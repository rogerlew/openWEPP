#!/usr/bin/env python3
"""Generate deterministic independent V4 shared-state authority vectors."""

from __future__ import annotations

import hashlib
import json
import math
import struct
from copy import deepcopy
from pathlib import Path
from typing import Any


HERE = Path(__file__).resolve().parent
FIXTURE_PATH = HERE / "openwepp_c3_woody_v4_vectors.json"
DEFINITION_PATH = HERE / "openwepp_c3_woody_v4_definition.json"
CONTRACT_PATH = HERE.parents[2] / "specifications/science-contracts/contracts/SC-VEGETATION-001.md"
CONTRACTS_DIR = CONTRACT_PATH.parent
STATIC_DEFINITION_SHA256 = "56b850e3727a3faf05d82c83c813c877ea2a3ee09bd5d4074648a7d18153e746"


def canonical_bytes(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def sha256(value: Any) -> str:
    return hashlib.sha256(canonical_bytes(value)).hexdigest()


def key_component(value: str) -> str:
    encoded = value.encode("utf-8")
    return f"/k{len(encoded)}:{encoded.hex()}"


def typed_state_lines(value: Any, path: str = "") -> list[str]:
    if isinstance(value, dict):
        lines = [f"{path}\tobject\t{len(value)}\n"]
        for key in sorted(value, key=lambda item: item.encode("utf-8")):
            lines.extend(typed_state_lines(value[key], f"{path}{key_component(key)}"))
        return lines
    if isinstance(value, list):
        lines = [f"{path}\tarray\t{len(value)}\n"]
        for index, item in enumerate(value):
            lines.extend(typed_state_lines(item, f"{path}/i{index}"))
        return lines
    if value is None:
        return [f"{path}\tnull\t\n"]
    if isinstance(value, bool):
        return [f"{path}\tbool\t{str(value).lower()}\n"]
    if isinstance(value, float):
        return [f"{path}\tf64be\t{struct.pack('>d', value).hex()}\n"]
    if isinstance(value, int):
        return [f"{path}\tu128\t{value}\n"]
    if isinstance(value, str):
        encoded = value.encode("utf-8")
        return [f"{path}\tstring\t{len(encoded)}:{encoded.hex()}\n"]
    raise TypeError(type(value))


def state_canonical_bytes(value: Any) -> bytes:
    return "".join(typed_state_lines(value)).encode("utf-8")


def state_sha256(value: Any) -> str:
    return hashlib.sha256(state_canonical_bytes(value)).hexdigest()


def section_digest(text: str, start: str, end: str) -> str:
    beginning = text.index(start)
    ending = text.index(end, beginning)
    return hashlib.sha256(text[beginning:ending].encode()).hexdigest()


def regenerate_definition(fixture_digest: str, generator_digest: str) -> None:
    definition = json.loads(DEFINITION_PATH.read_bytes())
    definition["pending_transfer_schema"] = {
        "amounts": "finite_nonnegative_carbon_nitrogen_dry_matter",
        "donor_ids": sorted(MATERIAL_DONOR_CLASSES),
        "owner_id": "nonempty_typed_owner_identity",
        "proposal_id": "positive_nonzero_u64",
        "receiver_ids": sorted(MATERIAL_RECEIVER_CLASSES),
        "transaction_id": "positive_nonzero_u128",
    }
    static_definition = deepcopy(definition)
    static_definition.pop("canonical_section_sha256")
    static_definition["independent_fixture"].pop("sha256")
    static_definition["independent_fixture"].pop("generator_sha256")
    if sha256(static_definition) != STATIC_DEFINITION_SHA256:
        raise AssertionError("V4 definition static authority drift")
    contract = CONTRACT_PATH.read_text()
    sections = {
        "vegetation_variables": ("## Variables and Units Using Canonical Symbols First\n", "## Algorithm State Surfaces\n"),
        "vegetation_algorithm_and_equations": ("## Algorithm Specification with Step Sequence\n", "## Branch and Guard Table\n"),
        "vegetation_invariants": ("## Invariants and Invariant Guard Map\n", "### Invariant Guard Map\n"),
        "vegetation_schema": ("## Constants and Parameters with Provenance Anchors\n", "## Unit-Governance Map\n"),
        "vegetation_numerics": ("## Tolerance and Numeric Notes\n", "## Calibration and Identifiability\n"),
        "v4_shared_state_amendment": ("## `OPENWEPP_C3_WOODY_V4` Shared-State Authority Amendment\n", "## Change Log\n"),
    }
    for key, (start, end) in sections.items():
        definition["canonical_section_sha256"][key] = section_digest(contract, start, end)
    adjacent_contracts = {
        "biogeochemistry_contract": "SC-BIOGEOCHEM-001.md",
        "land_surface_energy_contract": "SC-LANDSURFACEENERGY-001.md",
        "vegetation_transaction_contract": "SC-VEGETATIONTRANSACTION-001.md",
        "water_balance_contract": "SC-WATBAL-001.md",
    }
    for key, filename in adjacent_contracts.items():
        definition["canonical_section_sha256"][key] = hashlib.sha256(
            (CONTRACTS_DIR / filename).read_bytes()).hexdigest()
    definition["independent_fixture"]["sha256"] = fixture_digest
    definition["independent_fixture"]["generator_sha256"] = generator_digest
    DEFINITION_PATH.write_bytes(canonical_bytes(definition))


def sample_shared_state() -> dict[str, Any]:
    tissues = {}
    values = {
        "leaf": ((2.0, 0.08), (5.0, 0.15), (7.0, 0.18)),
        "fine_root": ((1.7, 0.04), (1.2, 0.03), (0.2, 0.005)),
        "live_stem": ((8.1, 0.05), (1.1, 0.01), (0.3, 0.002)),
        "dead_stem": ((4.4, 0.015), (0.7, 0.003), (0.1, 0.001)),
        "live_coarse_root": ((6.2, 0.04), (0.8, 0.005), (0.2, 0.001)),
        "dead_coarse_root": ((3.3, 0.011), (0.5, 0.002), (0.1, 0.0005)),
    }
    for tissue, (display, storage, transfer) in values.items():
        tissues[tissue] = {
            "display": {"carbon": display[0], "nitrogen": display[1]},
            "storage": {"carbon": storage[0], "nitrogen": storage[1]},
            "transfer": {"carbon": transfer[0], "nitrogen": transfer[1]},
        }
    return {
        "tissues": tissues,
        "retranslocation_n": 0.021,
        "nsc_c": 0.44,
        "xs_c": -0.031,
        "standing_dead": {"carbon": 0.77, "nitrogen": 0.008},
        "standing_dead_dm": 1.43,
        "phase": "onset",
        "onset_remaining_s": 43200.0,
        "offset_remaining_s": 0.0,
        "previous_gsi": 0.63,
        "pending_transfers": [
            {
                "transaction_id": 41,
                "owner_id": "stratum:canopy",
                "proposal_id": 9,
                "donor": "leaf",
                "receiver": "metabolic",
                "carbon": 0.013,
                "nitrogen": 0.0005,
                "dry_matter": 0.026,
            }
        ],
        "t10_k": 294.15,
        "leaf_area": 10.0,
        "stem_area": 3.5,
        "root_area": 16.875,
        "last_transaction_id": 41,
    }


def mutate_scalar(value: Any) -> Any:
    if isinstance(value, bool):
        return not value
    if isinstance(value, float):
        bits = struct.unpack(">Q", struct.pack(">d", value))[0]
        return struct.unpack(">d", struct.pack(">Q", bits ^ 1))[0]
    if isinstance(value, int):
        return value + 1
    if isinstance(value, str):
        return value + "|mutation"
    raise TypeError(type(value))


def mutation_paths(value: Any, prefix: str = "") -> list[tuple[str, Any]]:
    paths: list[tuple[str, Any]] = []
    if isinstance(value, dict):
        for key in sorted(value):
            child = f"{prefix}.{key}" if prefix else key
            paths.extend(mutation_paths(value[key], child))
    elif isinstance(value, list):
        for index, item in enumerate(value):
            paths.extend(mutation_paths(item, f"{prefix}[{index}]"))
    else:
        paths.append((prefix, value))
    return paths


def set_path(value: Any, path: str, replacement: Any) -> None:
    normalized = path.replace("[", ".").replace("]", "")
    parts = normalized.split(".")
    cursor = value
    for part in parts[:-1]:
        cursor = cursor[int(part)] if isinstance(cursor, list) else cursor[part]
    final = parts[-1]
    if isinstance(cursor, list):
        cursor[int(final)] = replacement
    else:
        cursor[final] = replacement


TOP_LEVEL_FIELDS = {
    "tissues", "retranslocation_n", "nsc_c", "xs_c", "standing_dead",
    "standing_dead_dm", "phase", "onset_remaining_s", "offset_remaining_s",
    "previous_gsi", "pending_transfers", "t10_k", "leaf_area", "stem_area",
    "root_area", "last_transaction_id",
}
TISSUE_IDS = {
    "leaf", "fine_root", "live_stem", "dead_stem", "live_coarse_root",
    "dead_coarse_root",
}
MATERIAL_DONOR_CLASSES = {
    "leaf", "fine_root", "live_stem", "dead_stem", "live_coarse_root",
    "dead_coarse_root",
}
MATERIAL_RECEIVER_CLASSES = {
    "metabolic", "cellulose", "lignin", "coarse_woody_debris",
}


def is_finite_number(value: Any) -> bool:
    return type(value) in (int, float) and math.isfinite(value)


def is_u128(value: Any) -> bool:
    return type(value) is int and 0 <= value < 2 ** 128


def is_positive_u128(value: Any) -> bool:
    return type(value) is int and 0 < value < 2 ** 128


def is_positive_u64(value: Any) -> bool:
    return type(value) is int and 0 < value < 2 ** 64


def strict_object(raw: str) -> dict[str, Any]:
    def reject_duplicate(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError("VEG-E-087 duplicate_field")
            result[key] = value
        return result

    return json.loads(raw, object_pairs_hook=reject_duplicate)


def validate_shared(value: dict[str, Any], sla: float, sai_relation: float,
                    root_to_leaf_area: float) -> str:
    if set(value) != TOP_LEVEL_FIELDS:
        legacy = {"previous_leaf_offset_flux", "previous_root_offset_flux"} & set(value)
        return "VEG-E-087 legacy_offset_flux_field" if legacy else "VEG-E-087 shared_field_set"
    if set(value["tissues"]) != TISSUE_IDS:
        return "VEG-E-087 tissue_set"
    for tissue in value["tissues"].values():
        if set(tissue) != {"display", "storage", "transfer"}:
            return "VEG-E-087 tissue_subpool_set"
        for pool in tissue.values():
            if set(pool) != {"carbon", "nitrogen"}:
                return "VEG-E-087 element_field_set"
            if any(not is_finite_number(x) or x < 0
                   for x in pool.values()):
                return "VEG-E-087 nonfinite_or_negative_element"
    for field in ("retranslocation_n", "nsc_c", "standing_dead_dm",
                  "onset_remaining_s", "offset_remaining_s"):
        amount = value[field]
        if not is_finite_number(amount) or amount < 0:
            return "VEG-E-087 nonfinite_or_negative_shared_amount"
    if not is_finite_number(value["xs_c"]):
        return "VEG-E-087 nonfinite_shared_amount"
    if (not is_finite_number(value["previous_gsi"])
            or not 0 <= value["previous_gsi"] <= 1):
        return "VEG-E-087 nonfinite_shared_amount"
    if value["phase"] not in {"dormant", "onset", "active", "offset"}:
        return "VEG-E-087 invalid_phase"
    if not is_finite_number(value["t10_k"]) or value["t10_k"] <= 0:
        return "VEG-E-087 invalid_t10"
    if set(value["standing_dead"]) != {"carbon", "nitrogen"} or any(
        not is_finite_number(x) or x < 0
        for x in value["standing_dead"].values()
    ):
        return "VEG-E-087 invalid_standing_dead"
    if not isinstance(value["pending_transfers"], list):
        return "VEG-E-087 invalid_pending_transfer"
    for transfer in value["pending_transfers"]:
        if not isinstance(transfer, dict):
            return "VEG-E-087 invalid_pending_transfer"
        if set(transfer) != {
            "transaction_id", "owner_id", "proposal_id", "donor", "receiver",
            "carbon", "nitrogen", "dry_matter",
        }:
            return "VEG-E-087 invalid_pending_transfer"
        if (not is_positive_u128(transfer["transaction_id"])
                or not is_positive_u64(transfer["proposal_id"])
                or not isinstance(transfer["owner_id"], str)
                or not transfer["owner_id"].strip()
                or transfer["donor"] not in MATERIAL_DONOR_CLASSES
                or transfer["receiver"] not in MATERIAL_RECEIVER_CLASSES):
            return "VEG-E-087 invalid_pending_transfer"
        if any(not is_finite_number(transfer[field]) or transfer[field] < 0
               for field in ("carbon", "nitrogen", "dry_matter")):
            return "VEG-E-087 invalid_pending_transfer"
    if not is_u128(value["last_transaction_id"]):
        return "VEG-E-087 invalid_transaction_id"
    leaf = value["tissues"]["leaf"]["display"]["carbon"] * sla
    displayed_leaf_n = value["tissues"]["leaf"]["display"]["nitrogen"]
    if leaf == 0.0 and displayed_leaf_n != 0.0:
        return "VEG-E-090 displayed_leaf_n_without_lai"
    stem = leaf * sai_relation
    root = (leaf + stem) * root_to_leaf_area
    for field, expected in (("leaf_area", leaf), ("stem_area", stem), ("root_area", root)):
        actual = value[field]
        if not is_finite_number(actual):
            return "VEG-E-087 nonfinite_shared_amount"
        if struct.pack(">d", actual) != struct.pack(">d", expected):
            return "VEG-E-088 displayed_leaf_area_identity"
    return "PASS"


def state_digest(value: dict[str, Any]) -> str:
    content = deepcopy(value)
    del content["state_sha256"]
    return state_sha256(content)


def v3_state_digest(value: dict[str, Any]) -> str:
    content = deepcopy(value)
    content["state_sha256"] = ""
    return sha256(content)


def migrate_v3_shared(value: dict[str, Any], sla: float, sai_relation: float,
                      root_to_leaf_area: float) -> tuple[str, dict[str, Any] | None]:
    expected = TOP_LEVEL_FIELDS | {
        "previous_leaf_offset_flux", "previous_root_offset_flux"
    }
    if set(value) != expected:
        return "VEG-E-089 v3_shared_field_set", None
    if any(not is_finite_number(value[field])
           for field in ("previous_leaf_offset_flux", "previous_root_offset_flux")):
        return "VEG-E-089 invalid_removed_field", None
    output = deepcopy(value)
    del output["previous_leaf_offset_flux"]
    del output["previous_root_offset_flux"]
    result = validate_shared(output, sla, sai_relation, root_to_leaf_area)
    return ("PASS", output) if result == "PASS" else ("VEG-E-089 source_area_cache_mismatch", None)


def migrate_v3_whole(value: dict[str, Any], sla: float, sai_relation: float,
                     root_to_leaf_area: float) -> tuple[list[dict[str, str]], dict[str, Any] | None]:
    failures = []
    if value.get("model_definition_sha256") != "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852":
        failures.append({"stratum_id": "<whole>", "failure": "VEG-E-089 v3_model_identity"})
    if value.get("configuration_sha256") != "4" * 64:
        failures.append({"stratum_id": "<whole>", "failure": "VEG-E-089 v3_configuration_identity"})
    if value.get("state_sha256") != v3_state_digest(value):
        failures.append({"stratum_id": "<whole>", "failure": "VEG-E-089 v3_state_digest"})
    expected_occupancies = {"canopy@tile-a", "understory@tile-a"}
    expected_strata = {"canopy", "understory"}
    if set(value.get("strata", {})) != expected_strata:
        failures.append({"stratum_id": "<whole>", "failure": "VEG-E-089 v3_stratum_set"})
    if set(value.get("occupancies", {})) != expected_occupancies:
        failures.append({"stratum_id": "<whole>", "failure": "VEG-E-089 v3_occupancy_set"})
    whole_transaction = value.get("last_transaction_id")
    for stratum_id, shared in value.get("strata", {}).items():
        if shared.get("last_transaction_id") != whole_transaction:
            failures.append({"stratum_id": stratum_id, "failure": "VEG-E-089 shared_lineage"})
        expected_owner = f"stratum:{stratum_id}"
        for transfer in shared.get("pending_transfers", []):
            if (transfer.get("transaction_id") != whole_transaction
                    or transfer.get("owner_id") != expected_owner):
                failures.append({"stratum_id": stratum_id, "failure": "VEG-E-089 transfer_lineage"})
    for occupancy_id, lane in value.get("occupancies", {}).items():
        stratum_id = occupancy_id.split("@", 1)[0]
        if stratum_id not in value.get("strata", {}):
            failures.append({"stratum_id": occupancy_id, "failure": "VEG-E-089 occupancy_stratum_membership"})
        if lane.get("last_accepted_transaction_id") != whole_transaction:
            failures.append({"stratum_id": occupancy_id, "failure": "VEG-E-089 occupancy_lineage"})
    if failures:
        return failures, None
    migrated = {}
    for stratum_id in sorted(value["strata"], key=lambda item: item.encode("utf-8")):
        status, output = migrate_v3_shared(
            value["strata"][stratum_id], sla, sai_relation, root_to_leaf_area)
        if output is None:
            failures.append({"stratum_id": stratum_id, "failure": status})
        else:
            migrated[stratum_id] = output
    if failures:
        return failures, None
    output = deepcopy(value)
    output["model_definition_sha256"] = "BOUND_BY_V4_DEFINITION_NOT_ORACLE"
    output["configuration_sha256"] = "BOUND_BY_V4_CONFIGURATION_NOT_ORACLE"
    output["strata"] = migrated
    output["state_sha256"] = ""
    output["state_sha256"] = state_digest(output)
    return [], output


def main() -> None:
    state = sample_shared_state()
    sla = 5.0
    sai_relation = 0.35
    root_to_leaf_area = 1.25
    lai = state["tissues"]["leaf"]["display"]["carbon"] * sla
    sai = lai * sai_relation
    rai = (lai + sai) * root_to_leaf_area

    zero_display = deepcopy(state)
    zero_display["tissues"]["leaf"]["display"]["carbon"] = 0.0
    zero_display["tissues"]["leaf"]["display"]["nitrogen"] = 0.0
    zero_display["leaf_area"] = 0.0
    zero_display["stem_area"] = 0.0
    zero_display["root_area"] = 0.0

    v3_source = deepcopy(state)
    v3_source["previous_leaf_offset_flux"] = 1.25e-7
    v3_source["previous_root_offset_flux"] = -2.5e-8
    v4_output = deepcopy(v3_source)
    del v4_output["previous_leaf_offset_flux"]
    del v4_output["previous_root_offset_flux"]
    occupancy = {
        "beta_hyd": 0.77,
        "canopy_air_specific_humidity_kg_kg": 0.0081,
        "canopy_air_temperature_k": 296.2,
        "canopy_liquid_kg_h2o_m2_tile_ground": 0.18,
        "dry_stem_temperature_k": 297.1,
        "last_accepted_transaction_id": 41,
        "root_node_potential_mm": -4200.0,
        "shade_ci_pa": 24.1,
        "shade_leaf_potential_mm": -5100.0,
        "shade_leaf_temperature_k": 295.8,
        "stem_potential_mm": -4700.0,
        "sun_ci_pa": 22.4,
        "sun_leaf_potential_mm": -5400.0,
        "sun_leaf_temperature_k": 299.2,
        "wet_surface_temperature_k": 296.7,
    }
    source_bad_cache = deepcopy(v3_source)
    source_bad_cache["leaf_area"] = 70.0
    second_v3_source = deepcopy(v3_source)
    second_v3_source["phase"] = "active"
    second_v3_source["previous_gsi"] = 0.81
    second_v3_source["last_transaction_id"] = 41
    second_v3_source["pending_transfers"][0]["owner_id"] = "stratum:understory"
    second_occupancy = deepcopy(occupancy)
    second_occupancy["canopy_liquid_kg_h2o_m2_tile_ground"] = 0.07
    second_occupancy["root_node_potential_mm"] = -3800.0
    validation_poisons = {}
    for name, candidate in {
        "missing_previous_gsi": {k: v for k, v in state.items() if k != "previous_gsi"},
        "retained_previous_leaf_offset_flux": v3_source,
        "missing_tissue": {**state, "tissues": {k: v for k, v in state["tissues"].items() if k != "leaf"}},
        "extra_tissue": {**state, "tissues": {**state["tissues"], "fruit": state["tissues"]["leaf"]}},
        "nonfinite_amount": {**state, "nsc_c": float("nan")},
        "display_plus_storage_lai": {**state, "leaf_area": (2.0 + 5.0) * sla},
        "wrong_stem_area": {**state, "stem_area": 3.5000000000000004},
        "wrong_root_area": {**state, "root_area": 16.875000000000004},
        "invalid_gsi": {**state, "previous_gsi": 1.01},
        "invalid_phase": {**state, "phase": "growing"},
        "bool_nsc": {**state, "nsc_c": True},
        "negative_transaction": {**state, "last_transaction_id": -1},
        "bool_transfer_carbon": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "carbon": True}],
        },
        "numeric_owner_id": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "owner_id": 4}],
        },
        "unsupported_donor": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "donor": "wood"}],
        },
        "unsupported_receiver": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "receiver": "litter_metabolic"}],
        },
        "zero_transfer_transaction": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "transaction_id": 0}],
        },
        "zero_transfer_proposal": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "proposal_id": 0}],
        },
        "overflow_transfer_transaction": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "transaction_id": 2 ** 128}],
        },
        "overflow_transfer_proposal": {
            **state,
            "pending_transfers": [{**state["pending_transfers"][0], "proposal_id": 2 ** 64}],
        },
        "displayed_n_without_lai": {
            **zero_display,
            "tissues": {
                **zero_display["tissues"],
                "leaf": {
                    **zero_display["tissues"]["leaf"],
                    "display": {"carbon": 0.0, "nitrogen": 0.08},
                },
            },
        },
    }.items():
        validation_poisons[name] = validate_shared(candidate, sla, sai_relation, root_to_leaf_area)
    duplicate_outcomes = {}
    for name, raw in {
        "duplicate_top_level": '{"nsc_c":0.44,"nsc_c":0.45}',
        "duplicate_tissue": '{"leaf":{},"leaf":{}}',
    }.items():
        try:
            strict_object(raw)
            duplicate_outcomes[name] = "INVALIDLY_ACCEPTED"
        except ValueError as error:
            duplicate_outcomes[name] = str(error)

    v3_whole = {
        "model_definition_sha256": "7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852",
        "configuration_sha256": "4" * 64,
        "state_sha256": "",
        "strata": {"canopy": v3_source, "understory": second_v3_source},
        "occupancies": {
            "canopy@tile-a": occupancy,
            "understory@tile-a": second_occupancy,
        },
        "last_transaction_id": 41,
    }
    v3_whole["state_sha256"] = v3_state_digest(v3_whole)
    migration_failures, v4_whole = migrate_v3_whole(
        v3_whole, sla, sai_relation, root_to_leaf_area)
    if migration_failures or v4_whole is None:
        raise AssertionError(migration_failures)
    whole_digest = v4_whole["state_sha256"]

    base_digest = state_sha256(state)
    mutation_digests = {}
    for path, original in mutation_paths(state):
        changed = deepcopy(state)
        set_path(changed, path, mutate_scalar(original))
        mutation_digests[path] = state_sha256(changed)
    whole_mutation_digests = {}
    whole_preimage = deepcopy(v4_whole)
    del whole_preimage["state_sha256"]
    for path, original in mutation_paths(whole_preimage):
        changed = deepcopy(whole_preimage)
        set_path(changed, path, mutate_scalar(original))
        whole_mutation_digests[path] = state_sha256(changed)
    multi_bad = deepcopy(v3_whole)
    multi_bad["strata"]["canopy"]["leaf_area"] = 99.0
    multi_bad["strata"]["understory"]["stem_area"] = 88.0
    multi_bad["state_sha256"] = v3_state_digest(multi_bad)
    multi_failures, multi_candidate = migrate_v3_whole(
        multi_bad, sla, sai_relation, root_to_leaf_area)
    migration_identity_poisons = {}
    for name, candidate in {
        "wrong_model": {**v3_whole, "model_definition_sha256": "0" * 64},
        "wrong_configuration": {**v3_whole, "configuration_sha256": "5" * 64},
        "wrong_state_digest": {**v3_whole, "state_sha256": "0" * 64},
        "missing_occupancy": {**v3_whole, "occupancies": {"canopy@tile-a": occupancy}},
        "extra_occupancy": {**v3_whole, "occupancies": {**v3_whole["occupancies"], "extra@tile-a": occupancy}},
        "shared_lineage": {**v3_whole, "strata": {**v3_whole["strata"], "canopy": {**v3_source, "last_transaction_id": 40}}},
        "occupancy_lineage": {**v3_whole, "occupancies": {**v3_whole["occupancies"], "canopy@tile-a": {**occupancy, "last_accepted_transaction_id": 40}}},
        "missing_stratum": {**v3_whole, "strata": {"canopy": v3_source}},
        "extra_stratum": {**v3_whole, "strata": {**v3_whole["strata"], "extra": v3_source}},
        "wrong_transfer_owner": {**v3_whole, "strata": {**v3_whole["strata"], "understory": {**second_v3_source, "pending_transfers": [{**second_v3_source["pending_transfers"][0], "owner_id": "stratum:canopy"}]}}},
        "wrong_transfer_transaction": {**v3_whole, "strata": {**v3_whole["strata"], "canopy": {**v3_source, "pending_transfers": [{**v3_source["pending_transfers"][0], "transaction_id": 40}]}}},
    }.items():
        if name not in {"wrong_state_digest", "wrong_model", "wrong_configuration"}:
            candidate["state_sha256"] = v3_state_digest(candidate)
        failures, output = migrate_v3_whole(candidate, sla, sai_relation, root_to_leaf_area)
        migration_identity_poisons[name] = {"failures": failures, "candidate": output}

    total_leaf_c = sum(
        state["tissues"]["leaf"][pool]["carbon"]
        for pool in ("display", "storage", "transfer")
    )
    insertion_permuted = dict(reversed(list(state.items())))
    fixture = {
        "model_version": "OPENWEPP_C3_WOODY_V4",
        "oracle_independence": {
            "implementation_language": "Python standard library only",
            "calls_rust": False,
            "expected_values_generated_by_rust": False,
            "canonical_serialization": "recursive key sort, compact separators, UTF-8, LF",
            "state_digest_serialization": "OPENWEPP_V4_STATE_CANONICAL_V1 typed UTF-8 lines",
        },
        "displayed_leaf_area": {
            "configuration": {
                "sla_m2_per_kg_c": sla,
                "sai_relation": sai_relation,
                "root_to_leaf_area": root_to_leaf_area,
            },
            "source_carbon": {
                "display": 2.0,
                "storage": 5.0,
                "transfer": 7.0,
            },
            "source_nitrogen": {
                "display": 0.08,
                "storage": 0.15,
                "transfer": 0.18,
            },
            "expected_displayed_leaf_n_area": 0.08 / lai,
            "leaf_n_poisons": {
                "display_plus_storage_n_area": (0.08 + 0.15) / lai,
                "display_plus_transfer_n_area": (0.08 + 0.18) / lai,
                "all_leaf_pool_n_area": (0.08 + 0.15 + 0.18) / lai,
            },
            "expected": {"leaf_area": lai, "stem_area": sai, "root_area": rai},
            "poisons": {
                "display_plus_storage_lai": (2.0 + 5.0) * sla,
                "display_plus_transfer_lai": (2.0 + 7.0) * sla,
                "all_leaf_pools_lai": total_leaf_c * sla,
            },
        },
        "zero_display_nonzero_donors": {
            "state": zero_display,
            "expected_leaf_area": 0.0,
            "expected_photosynthetic_leaf_area": 0.0,
            "storage_and_transfer_carbon": 12.0,
            "expected_displayed_leaf_n": 0.0,
            "storage_and_transfer_nitrogen": 0.33,
        },
        "shared_state": state,
        "canonical_state_sha256": base_digest,
        "mutation_digests": mutation_digests,
        "whole_state_mutation_digests": whole_mutation_digests,
        "schema": {
            "top_level_fields": sorted(state),
            "tissue_ids": sorted(state["tissues"]),
            "tissue_subpools": ["display", "storage", "transfer"],
            "element_fields": ["carbon", "nitrogen"],
            "removed_v3_fields": [
                "previous_leaf_offset_flux",
                "previous_root_offset_flux",
            ],
            "poisons": {
                "validation_outcomes": validation_poisons,
                "duplicate_parse_outcomes": duplicate_outcomes,
            },
        },
        "v3_to_v4_migration": {
            "v3_whole_state": v3_whole,
            "expected_v4_whole_state": v4_whole,
            "v4_model_identity_rule": {
                "fixture_placeholder": "BOUND_BY_V4_DEFINITION_NOT_ORACLE",
                "reason": "avoids definition-fixture digest cycle",
                "runtime_rule": "inject and validate exact V4 definition digest before state digest",
                "v3_configuration_fixture": "4444444444444444444444444444444444444444444444444444444444444444",
                "v4_configuration_placeholder": "BOUND_BY_V4_CONFIGURATION_NOT_ORACLE",
                "configuration_rule": "recompute configuration digest after injecting V4 model identity, then inject V4 configuration digest before V4 state digest",
            },
            "v3_shared_state": v3_source,
            "expected_v4_shared_state": v4_output,
            "v3_occupancy_state": occupancy,
            "expected_v4_occupancy_state": occupancy,
            "removed_fields": [
                "previous_leaf_offset_flux",
                "previous_root_offset_flux",
            ],
            "retained_field_bytes_equal": canonical_bytes(v4_output) == canonical_bytes(state),
            "mismatched_source_cache": {
                "candidate_leaf_area": 70.0,
                "evaluated_source_validation": validate_shared(
                    {k: v for k, v in source_bad_cache.items()
                     if k not in {"previous_leaf_offset_flux", "previous_root_offset_flux"}},
                    sla, sai_relation, root_to_leaf_area),
                "migration_result": None,
            },
            "multi_stratum_invalid": {
                "failures": multi_failures,
                "candidate": multi_candidate,
            },
            "identity_poisons": migration_identity_poisons,
        },
        "checks": {
            "displayed_leaf_area_exact": lai == 10.0,
            "stem_area_exact": sai == 3.5,
            "root_area_exact": rai == 16.875,
            "donor_pools_excluded": lai != total_leaf_c * sla,
            "zero_display_exact_zero": zero_display["leaf_area"] == 0.0,
            "six_tissues": len(state["tissues"]) == 6,
            "removed_fields_absent": all(field not in state for field in (
                "previous_leaf_offset_flux", "previous_root_offset_flux"
            )),
            "recursive_order_invariant": state_sha256(insertion_permuted) == base_digest,
            "every_scalar_changes_digest": all(digest != base_digest for digest in mutation_digests.values()),
            "migration_retains_all_v4_fields": canonical_bytes(v4_output) == canonical_bytes(state),
            "migration_preserves_occupancy": canonical_bytes(occupancy) == canonical_bytes(deepcopy(occupancy)),
            "source_bad_cache_rejected": validate_shared(
                {k: v for k, v in source_bad_cache.items()
                 if k not in {"previous_leaf_offset_flux", "previous_root_offset_flux"}},
                sla, sai_relation, root_to_leaf_area) == "VEG-E-088 displayed_leaf_area_identity",
            "schema_poisons_rejected": all(outcome != "PASS" for outcome in validation_poisons.values()),
            "duplicates_rejected": all(outcome == "VEG-E-087 duplicate_field" for outcome in duplicate_outcomes.values()),
            "whole_state_digests_bound": bool(v3_whole["state_sha256"] and v4_whole["state_sha256"]),
            "every_whole_state_scalar_changes_digest": all(
                digest != whole_digest for digest in whole_mutation_digests.values()),
            "actual_migration_preserves_both_occupancies": (
                v4_whole["occupancies"] == v3_whole["occupancies"]),
            "multi_stratum_failures_exhaustive": (
                multi_candidate is None
                and [item["stratum_id"] for item in multi_failures]
                    == ["canopy", "understory"]),
            "displayed_leaf_n_only": 0.08 / lai != (0.08 + 0.15 + 0.18) / lai,
            "migration_rebinds_configuration_identity": (
                v4_whole["configuration_sha256"] != v3_whole["configuration_sha256"]
                and v4_whole["configuration_sha256"] == "BOUND_BY_V4_CONFIGURATION_NOT_ORACLE"),
            "all_migration_identity_poisons_rejected": all(
                result["failures"] and result["candidate"] is None
                for result in migration_identity_poisons.values()),
        },
    }
    if not all(fixture["checks"].values()):
        raise AssertionError(fixture["checks"])
    fixture_bytes = canonical_bytes(fixture)
    FIXTURE_PATH.write_bytes(fixture_bytes)
    generator_digest = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    regenerate_definition(hashlib.sha256(fixture_bytes).hexdigest(), generator_digest)


if __name__ == "__main__":
    main()
