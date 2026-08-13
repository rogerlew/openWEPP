#!/usr/bin/env python3
"""Independent standard-library oracle for V7 six-tissue onset authority."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from copy import deepcopy
from pathlib import Path
from typing import Callable

TISSUES = (
    "leaf",
    "fine_root",
    "live_stem",
    "dead_stem",
    "live_coarse_root",
    "dead_coarse_root",
)
ELEMENTS = ("carbon", "nitrogen")
OWNERS = ("display", "storage", "transfer")
F_STOR_XFER = 0.5
ONSET_THRESHOLD = 0.5
V6_DIGEST = "a5a5ed77b4672b97b7c50103089067d70ade03bc1b5aff4e08ba6fdffc05d426"
VECTOR_PATH = Path(__file__).with_name("openwepp_c3_woody_v7_vectors.json")


class OracleError(ValueError):
    """Typed rejection used only to prove alternative discrimination."""

    def __init__(self, category: str, evidence: dict | None = None) -> None:
        super().__init__(category)
        self.category = category
        self.evidence = evidence or {"alternative_executed": True, "differs": True}


def fixture_pools() -> dict[str, dict[str, dict[str, float]]]:
    nitrogen = (0.13, 0.071, 0.233, 0.089, 0.317, 0.149)
    return {
        tissue: {
            "display": {"carbon": 10.0 * index, "nitrogen": nitrogen[index - 1]},
            "storage": {
                "carbon": 2.0 * index,
                "nitrogen": nitrogen[-index] / 11.0,
            },
            "transfer": {
                "carbon": 0.3 * index,
                "nitrogen": nitrogen[(index + 1) % len(nitrogen)] / 37.0,
            },
        }
        for index, tissue in enumerate(TISSUES, start=1)
    }


def canonical_bytes(value: object) -> bytes:
    return (
        json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)
        + "\n"
    ).encode()


def sha256(value: object) -> str:
    return hashlib.sha256(canonical_bytes(value)).hexdigest()


def validate_pools(pools: dict) -> None:
    if tuple(pools) != TISSUES:
        raise OracleError("VEG-E-097")
    for tissue in TISSUES:
        if tuple(pools[tissue]) != OWNERS:
            raise OracleError("VEG-E-097")
        for owner in OWNERS:
            if tuple(pools[tissue][owner]) != ELEMENTS:
                raise OracleError("VEG-E-097")
            for value in pools[tissue][owner].values():
                if not math.isfinite(value) or value < 0.0:
                    raise OracleError("VEG-E-097")


def totals(pools: dict, tissue: str, element: str) -> float:
    return sum(pools[tissue][owner][element] for owner in OWNERS)


def prepare(beginning: dict) -> tuple[dict, dict]:
    validate_pools(beginning)
    candidate = deepcopy(beginning)
    moved: dict[str, dict[str, float]] = {}
    for tissue in TISSUES:
        moved[tissue] = {}
        for element in ELEMENTS:
            amount = F_STOR_XFER * beginning[tissue]["storage"][element]
            moved[tissue][element] = amount
            candidate[tissue]["storage"][element] = (
                beginning[tissue]["storage"][element] - amount
            )
            candidate[tissue]["transfer"][element] = (
                beginning[tissue]["transfer"][element] + amount
            )
    return candidate, moved


def onset_fraction(remaining_s: float, dt_s: float) -> float:
    if not math.isfinite(remaining_s) or not math.isfinite(dt_s):
        raise OracleError("VEG-E-097")
    if remaining_s <= 0.0 or dt_s <= 0.0:
        raise OracleError("VEG-E-097")
    return 1.0 if remaining_s <= dt_s else min(1.0, 2.0 * dt_s / remaining_s)


def deploy(prepared: dict, remaining_s: float, dt_s: float) -> tuple[dict, dict, float]:
    validate_pools(prepared)
    fraction = onset_fraction(remaining_s, dt_s)
    candidate = deepcopy(prepared)
    moved: dict[str, dict[str, float]] = {}
    for tissue in TISSUES:
        moved[tissue] = {}
        for element in ELEMENTS:
            amount = prepared[tissue]["transfer"][element] * fraction
            moved[tissue][element] = amount
            candidate[tissue]["transfer"][element] = (
                0.0
                if fraction == 1.0
                else prepared[tissue]["transfer"][element] - amount
            )
            candidate[tissue]["display"][element] = (
                prepared[tissue]["display"][element] + amount
            )
    return candidate, moved, fraction


def edge_action(
    phase: str, previous_gsi: float, current_gsi: float
) -> tuple[str, bool]:
    if not math.isfinite(previous_gsi) or not math.isfinite(current_gsi):
        raise OracleError("VEG-E-097")
    crossing = previous_gsi < ONSET_THRESHOLD and current_gsi > ONSET_THRESHOLD
    if phase == "dormant" and crossing:
        return "onset", True
    if phase == "onset":
        return "onset", False
    return phase, False


def all_transfer_empty(pools: dict) -> bool:
    return all(
        pools[tissue]["transfer"][element] == 0.0
        for tissue in TISSUES
        for element in ELEMENTS
    )


def next_phase(pools: dict) -> str:
    return "active" if all_transfer_empty(pools) else "onset"


def closure(beginning: dict, ending: dict) -> dict:
    return {
        tissue: {
            element: totals(beginning, tissue, element)
            - totals(ending, tissue, element)
            for element in ELEMENTS
        }
        for tissue in TISSUES
    }


def assert_zero_closure(beginning: dict, ending: dict) -> None:
    residuals = closure(beginning, ending)
    maximum = max(abs(value) for row in residuals.values() for value in row.values())
    if maximum > 1.0e-12:
        raise OracleError(
            "VEG-E-097",
            {
                "alternative_executed": True,
                "differs": True,
                "maximum_closure_residual": maximum,
                "threshold": 1.0e-12,
            },
        )


def validate_evergreen(fcur: float, pools: dict) -> None:
    validate_pools(pools)
    if fcur != 1.0:
        raise OracleError("VEG-E-098")
    if any(
        pools[tissue][owner][element] != 0.0
        for tissue in TISSUES
        for owner in ("storage", "transfer")
        for element in ELEMENTS
    ):
        raise OracleError("VEG-E-098")


def migrate_v6_to_v7(source: dict, mode: str, fcur: float) -> dict:
    if source["model_definition_sha256"] != V6_DIGEST:
        raise OracleError("VEG-E-099")
    validate_pools(source["pools"])
    if mode == "evergreen":
        unresolved = evergreen_unresolved_fields(fcur, source["pools"])
        if unresolved:
            return {
                "status": "incomplete",
                "typed_error": "VEG-E-098",
                "candidate": None,
                "unresolved": unresolved,
            }
    candidate = deepcopy(source)
    candidate["model_version"] = "OPENWEPP_C3_WOODY_V7"
    candidate["model_definition_sha256"] = "<V7_DEFINITION_SHA256>"
    candidate["configuration_sha256"] = "<V7_CONFIGURATION_SHA256>"
    candidate["state_sha256"] = sha256(
        {key: value for key, value in candidate.items() if key != "state_sha256"}
    )
    return candidate


def evergreen_unresolved_fields(fcur: float, pools: dict) -> list[dict]:
    unresolved = []
    if fcur != 1.0:
        unresolved.append({"field": "current_growth_fraction", "reason": "not_exact_one"})
    for tissue in TISSUES:
        for owner in ("storage", "transfer"):
            for element in ELEMENTS:
                if pools[tissue][owner][element] != 0.0:
                    unresolved.append(
                        {
                            "stratum": "stratum-1",
                            "tissue": tissue,
                            "pool": owner,
                            "element": element,
                            "reason": "evergreen_nonzero_pool",
                        }
                    )
    return unresolved


def rejection(name: str, operation: Callable[[], object], expected: str) -> dict:
    try:
        produced = operation()
    except OracleError as error:
        return {
            "name": name,
            "candidate": None,
            "typed_error": error.category,
            "expected_error": expected,
            "rejected": error.category == expected,
            "discriminator": error.evidence,
        }
    return {
        "name": name,
        "candidate_sha256": sha256(produced),
        "typed_error": None,
        "expected_error": expected,
        "rejected": False,
    }


def bad_prepare(
    beginning: dict,
    fraction: float = F_STOR_XFER,
    tissues: tuple[str, ...] = TISSUES,
    overwrite: bool = False,
    move_elements: tuple[str, ...] = ELEMENTS,
) -> dict:
    candidate = deepcopy(beginning)
    for tissue in tissues:
        for element in move_elements:
            amount = fraction * beginning[tissue]["storage"][element]
            candidate[tissue]["storage"][element] -= amount
            candidate[tissue]["transfer"][element] = (
                amount
                if overwrite
                else candidate[tissue]["transfer"][element] + amount
            )
    assert_zero_closure(beginning, candidate)
    canonical, _ = prepare(beginning)
    if candidate != canonical:
        raise OracleError("VEG-E-097")
    return candidate


def require_exact(candidate: object, expected: object) -> object:
    if candidate != expected:
        raise OracleError(
            "VEG-E-097",
            {
                "alternative_executed": True,
                "differs": True,
                "candidate_sha256": sha256(candidate),
                "canonical_sha256": sha256(expected),
            },
        )
    return candidate


def validate_preparation_candidate(
    beginning: dict,
    candidate: dict,
    *,
    additional_respiration: float = 0.0,
    mineral_n_request: float = 0.0,
) -> dict:
    canonical, _ = prepare(beginning)
    assert_zero_closure(beginning, candidate)
    if candidate != canonical:
        raise OracleError(
            "VEG-E-097",
            {
                "alternative_executed": True,
                "differs": True,
                "candidate_sha256": sha256(candidate),
                "canonical_sha256": sha256(canonical),
            },
        )
    if additional_respiration != 0.0 or mineral_n_request != 0.0:
        raise OracleError(
            "VEG-E-097",
            {
                "alternative_executed": True,
                "differs": True,
                "canonical_additional_respiration": 0.0,
                "alternative_additional_respiration": additional_respiration,
                "canonical_mineral_n_request": 0.0,
                "alternative_mineral_n_request": mineral_n_request,
            },
        )
    return candidate


def validate_phase_candidate(pools: dict, proposed_phase: str) -> str:
    canonical = next_phase(pools)
    if proposed_phase != canonical:
        raise OracleError(
            "VEG-E-097",
            {
                "alternative_executed": True,
                "differs": True,
                "canonical_phase": canonical,
                "alternative_phase": proposed_phase,
            },
        )
    return proposed_phase


def validate_preparation_event(
    beginning: dict,
    phase: str,
    previous_gsi: float,
    current_gsi: float,
    candidate: dict,
) -> dict:
    _, should_prepare = edge_action(phase, previous_gsi, current_gsi)
    expected = prepare(beginning)[0] if should_prepare else beginning
    if candidate != expected:
        raise OracleError(
            "VEG-E-097",
            {
                "alternative_executed": True,
                "differs": True,
                "candidate_sha256": sha256(candidate),
                "canonical_sha256": sha256(expected),
                "canonical_prepare": should_prepare,
            },
        )
    return candidate


def poison_vectors(beginning: dict) -> list[dict]:
    prepared, _ = prepare(beginning)
    with_current_growth = deepcopy(beginning)
    for tissue in TISSUES:
        with_current_growth[tissue]["storage"]["carbon"] += 0.6
        with_current_growth[tissue]["storage"]["nitrogen"] += 0.006

    operations: list[tuple[str, Callable[[], object], str]] = [
        ("move_100_percent", lambda: bad_prepare(beginning, fraction=1.0), "VEG-E-097"),
        ("move_only_leaf", lambda: bad_prepare(beginning, tissues=("leaf",)), "VEG-E-097"),
        (
            "move_leaf_and_fine_root_only",
            lambda: bad_prepare(beginning, tissues=("leaf", "fine_root")),
            "VEG-E-097",
        ),
        ("overwrite_existing_transfer", lambda: bad_prepare(beginning, overwrite=True), "VEG-E-097"),
        (
            "move_current_interval_storage",
            lambda: require_exact(bad_prepare(with_current_growth), prepared),
            "VEG-E-097",
        ),
        (
            "prepare_every_onset_timestep",
            lambda: require_exact(bad_prepare(prepared), prepared),
            "VEG-E-097",
        ),
        (
            "move_c_without_n",
            lambda: bad_prepare(beginning, move_elements=("carbon",)),
            "VEG-E-097",
        ),
        (
            "move_n_without_c",
            lambda: bad_prepare(beginning, move_elements=("nitrogen",)),
            "VEG-E-097",
        ),
        (
            "recompute_n_from_c_over_cn",
            lambda: validate_preparation_candidate(
                beginning,
                {
                    tissue: {
                        **prepared[tissue],
                        "transfer": {
                            **prepared[tissue]["transfer"],
                            "nitrogen": beginning[tissue]["transfer"]["nitrogen"]
                            + 0.5 * beginning[tissue]["storage"]["carbon"] / 42.0,
                        },
                    }
                    for tissue in TISSUES
                },
            ),
            "VEG-E-097",
        ),
        (
            "double_growth_respiration",
            lambda: validate_preparation_candidate(
                beginning, prepared, additional_respiration=0.12
            ),
            "VEG-E-097",
        ),
        (
            "request_mineral_n_for_stored_tissue",
            lambda: validate_preparation_candidate(
                beginning,
                prepared,
                mineral_n_request=sum(
                    prepared[t]["transfer"]["nitrogen"] for t in TISSUES
                ),
            ),
            "VEG-E-097",
        ),
        (
            "active_with_nonleaf_transfer",
            lambda: validate_phase_candidate(prepared, "active"),
            "VEG-E-097",
        ),
        (
            "calendar_year_transfer_without_onset",
            lambda: validate_preparation_event(
                beginning, "dormant", 0.2, 0.2, prepared
            ),
            "VEG-E-097",
        ),
        (
            "background_seasonal_transfer",
            lambda: validate_preparation_event(
                beginning, "active", 0.49, 0.51, prepared
            ),
            "VEG-E-097",
        ),
        (
            "evergreen_storage_accumulation",
            lambda: validate_evergreen(1.0, beginning),
            "VEG-E-098",
        ),
        (
            "negative_pool",
            lambda: prepare(
                {
                    **beginning,
                    "leaf": {
                        **beginning["leaf"],
                        "storage": {"carbon": -1.0, "nitrogen": 0.01},
                    },
                }
            ),
            "VEG-E-097",
        ),
        (
            "nonfinite_pool",
            lambda: prepare(
                {
                    **beginning,
                    "leaf": {
                        **beginning["leaf"],
                        "storage": {"carbon": float("inf"), "nitrogen": 0.01},
                    },
                }
            ),
            "VEG-E-097",
        ),
    ]
    return [rejection(name, operation, expected) for name, operation, expected in operations]


def rollback_vectors(beginning_owners: dict) -> list[dict]:
    before = canonical_bytes(beginning_owners)
    before_hash = hashlib.sha256(before).hexdigest()
    results = []
    for phase in (
        "after_preparation",
        "during_onset_deployment",
        "after_n_authorization",
        "during_allocation",
        "closure_validation",
        "before_commit",
    ):
        working = deepcopy(beginning_owners)
        prepared, _ = prepare(working["vegetation"]["pools"])
        working["vegetation"]["pools"] = prepared
        if phase != "after_preparation":
            deployed, _, _ = deploy(prepared, 345_600.0, 86_400.0)
            working["vegetation"]["pools"] = deployed
        if phase in ("after_n_authorization", "during_allocation", "closure_validation", "before_commit"):
            working["attempt"]["n_maximum_authorization"] = 0.01
        if phase in ("during_allocation", "closure_validation", "before_commit"):
            working["bgc"]["ammonium"][0] -= 0.01
            working["attempt"]["n_finalized_use"] = 0.01
            working["vegetation"]["pools"]["leaf"]["storage"]["carbon"] += 0.6
            working["vegetation"]["pools"]["leaf"]["storage"]["nitrogen"] += 0.006
        if phase in ("closure_validation", "before_commit"):
            working["attempt"]["closure_residual"] = closure(
                beginning_owners["vegetation"]["pools"],
                working["vegetation"]["pools"],
            )
        working["attempt"]["failure_phase"] = phase
        working["attempt"]["typed_error"] = (
            "VEG-E-100" if phase != "closure_validation" else "VEG-E-097"
        )
        # Every operation above is candidate-local. Rejection discards it.
        after = canonical_bytes(beginning_owners)
        results.append(
            {
                "phase": phase,
                "candidate": None,
                "before_sha256": before_hash,
                "after_sha256": hashlib.sha256(after).hexdigest(),
                "owners_byte_identical": before == after,
                "attempted_candidate_sha256": sha256(working),
            }
        )
    return results


def rollback_leak_poison(beginning_owners: dict) -> dict:
    leaked = deepcopy(beginning_owners)
    leaked["vegetation"]["pools"], _ = prepare(
        beginning_owners["vegetation"]["pools"]
    )
    before = canonical_bytes(beginning_owners)
    after = canonical_bytes(leaked)
    return {
        "name": "candidate_mutation_leaked_to_accepted_owner",
        "before_sha256": hashlib.sha256(before).hexdigest(),
        "after_sha256": hashlib.sha256(after).hexdigest(),
        "owners_byte_identical": before == after,
        "accepted": before == after,
        "typed_error": "VEG-E-100",
    }


def build() -> dict:
    beginning = fixture_pools()
    prepared, prep = prepare(beginning)
    first, first_deployed, first_fraction = deploy(
        prepared, remaining_s=345_600.0, dt_s=86_400.0
    )
    second, second_deployed, second_fraction = deploy(
        first, remaining_s=259_200.0, dt_s=86_400.0
    )
    final, final_deployed, final_fraction = deploy(
        second, remaining_s=172_800.0, dt_s=86_400.0
    )
    terminal_source = deepcopy(second)
    terminal_source["dead_coarse_root"]["transfer"]["carbon"] = 5e-15
    terminal_source["dead_coarse_root"]["transfer"]["nitrogen"] = 7e-16
    terminal_final, terminal_moved, terminal_fraction = deploy(
        terminal_source, remaining_s=86_400.0, dt_s=86_400.0
    )
    assert_zero_closure(beginning, prepared)
    assert_zero_closure(prepared, first)
    assert_zero_closure(first, second)
    assert_zero_closure(second, final)

    allocation = {tissue: {"carbon": 0.6, "nitrogen": 0.006} for tissue in TISSUES}
    same_interval = deepcopy(first)
    for tissue in TISSUES:
        for element in ELEMENTS:
            same_interval[tissue]["storage"][element] += allocation[tissue][element]

    v6_state = {
        "model_version": "OPENWEPP_C3_WOODY_V6",
        "model_definition_sha256": V6_DIGEST,
        "configuration_sha256": "v6-configuration",
        "state_sha256": "v6-state",
        "phase": "dormant",
        "pools": beginning,
        "onset_remaining_s": 345_600.0,
        "offset_remaining_s": 0.0,
        "previous_gsi": 0.49,
        "current_gsi": 0.51,
        "t10_k": 283.15,
        "nsc_c": 1.7,
        "xs_c": -0.02,
        "retranslocation_n": 0.013,
        "last_transaction_id": 41,
        "occupancies": {"stratum-1/tile-1": {"canopy_liquid": 0.2}},
        "pending_transfers": [{"receiver": "litter", "carbon": 0.1}],
        "diagnostics": {"previous_onset_count": 0, "last_closure": 0.0},
    }
    migrated = migrate_v6_to_v7(v6_state, "seasonal_deciduous", 0.4)
    identity_keys = {
        "model_version",
        "model_definition_sha256",
        "configuration_sha256",
        "state_sha256",
    }
    source_nonidentity = {
        key: value for key, value in v6_state.items() if key not in identity_keys
    }
    migrated_nonidentity = {
        key: value for key, value in migrated.items() if key not in identity_keys
    }

    zero_pools = fixture_pools()
    for tissue in TISSUES:
        for owner in ("storage", "transfer"):
            for element in ELEMENTS:
                zero_pools[tissue][owner][element] = -0.0
    validate_evergreen(1.0, zero_pools)

    owner_snapshot = {
        "vegetation": v6_state,
        "water": {"store": [3.0, 4.0], "transaction_id": 41},
        "bgc": {"ammonium": [0.2], "nitrate": [0.3], "transaction_id": 41},
        "energy": {"beginning_j_m2": 12.0, "transaction_id": 41},
        "attempt": {"failure_phase": None, "transaction_id": 42},
    }

    return {
        "model_version": "OPENWEPP_C3_WOODY_V7",
        "oracle_independence": {
            "implementation_language": "Python standard library only",
            "calls_rust": False,
            "expected_values_generated_by_rust": False,
            "canonical_serialization": "recursive key sort, compact separators, UTF-8, LF",
        },
        "constants": {
            "f_stor_xfer": F_STOR_XFER,
            "onset_threshold": ONSET_THRESHOLD,
            "tissues": list(TISSUES),
            "elements": list(ELEMENTS),
        },
        "event_branches": [
            {
                "name": name,
                "phase_after_detection": edge_action(phase, previous, current)[0],
                "prepare": edge_action(phase, previous, current)[1],
            }
            for name, phase, previous, current in (
                ("upward_crossing", "dormant", 0.49, 0.51),
                ("threshold_equality", "dormant", 0.49, 0.5),
                ("no_upward_crossing", "dormant", 0.51, 0.6),
                ("already_onset", "onset", 0.49, 0.51),
                ("active_phase", "active", 0.49, 0.51),
            )
        ],
        "six_tissue_vectors": [
            {
                "tissue": tissue,
                "beginning": beginning[tissue],
                "prepared": prepared[tissue],
                "preparation_amount": prep[tissue],
                "first_deployment_amount": first_deployed[tissue],
                "first_ending": first[tissue],
                "final": final[tissue],
                "same_interval_ending_storage": same_interval[tissue]["storage"],
                "carbon_source_operand_independent": prep[tissue]["carbon"]
                == 0.5 * beginning[tissue]["storage"]["carbon"],
                "nitrogen_source_operand_independent": prep[tissue]["nitrogen"]
                == 0.5 * beginning[tissue]["storage"]["nitrogen"],
            }
            for tissue in TISSUES
        ],
        "preparation_closure": closure(beginning, prepared),
        "first_onset_interval": {
            "remaining_s": 345_600.0,
            "dt_s": 86_400.0,
            "fraction": first_fraction,
            "closure": closure(prepared, first),
        },
        "multi_interval_onset": {
            "fractions": [first_fraction, second_fraction, final_fraction],
            "deployment_amounts": [first_deployed, second_deployed, final_deployed],
            "preparation_count": 1,
            "all_transfer_exact_zero": all_transfer_empty(final),
            "phase_before_exhaustion": next_phase(second),
            "final_phase": next_phase(final),
            "three_intervals_executed": True,
            "closure": closure(beginning, final),
        },
        "terminal_remainder_branch": {
            "remaining_s": 86_400.0,
            "dt_s": 86_400.0,
            "fraction": terminal_fraction,
            "moved": terminal_moved,
            "all_transfer_exact_zero": all_transfer_empty(terminal_final),
            "phase": next_phase(terminal_final),
        },
        "current_interval_allocation_exclusion": {
            "allocation_after_onset_per_tissue": allocation["leaf"],
            "preparation_source_sha256": sha256(beginning),
            "post_allocation_sha256": sha256(same_interval),
            "new_allocation_remains_in_storage": all(
                same_interval[tissue]["storage"][element]
                == first[tissue]["storage"][element] + allocation[tissue][element]
                for tissue in TISSUES
                for element in ELEMENTS
            ),
        },
        "evergreen": {
            "accepted_signed_zero_sha256": sha256(zero_pools),
            "rejected": [
                rejection(
                    "fcur_not_exact_one",
                    lambda: validate_evergreen(0.9999999999999999, zero_pools),
                    "VEG-E-098",
                ),
                rejection(
                    "nonzero_storage",
                    lambda: validate_evergreen(1.0, beginning),
                    "VEG-E-098",
                ),
            ],
        },
        "migration": {
            "seasonal_nonidentity_fields_preserved": (
                source_nonidentity == migrated_nonidentity
            ),
            "source_nonidentity_sha256": sha256(source_nonidentity),
            "migrated_nonidentity_sha256": sha256(migrated_nonidentity),
            "preparation_not_executed": migrated["pools"] == beginning,
            "identity_fields_changed": [
                "model_version",
                "model_definition_sha256",
                "configuration_sha256",
                "state_sha256",
            ],
            "evergreen_incomplete": migrate_v6_to_v7(
                v6_state, "evergreen", 0.75
            ),
            "evergreen_unresolved_field_count": len(
                migrate_v6_to_v7(v6_state, "evergreen", 0.75)["unresolved"]
            ),
            "invalid_source": rejection(
                "wrong_v6_digest",
                lambda: migrate_v6_to_v7(
                    {**v6_state, "model_definition_sha256": "wrong"},
                    "seasonal_deciduous",
                    0.4,
                ),
                "VEG-E-099",
            ),
        },
        "poisons": poison_vectors(beginning),
        "rollback_injections": rollback_vectors(owner_snapshot),
        "rollback_leak_poison": rollback_leak_poison(owner_snapshot),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true")
    arguments = parser.parse_args()
    generated = canonical_bytes(build())
    if arguments.verify:
        committed = VECTOR_PATH.read_bytes()
        if committed != generated:
            print(
                f"fixture mismatch: expected {hashlib.sha256(generated).hexdigest()} "
                f"found {hashlib.sha256(committed).hexdigest()}"
            )
            return 1
        print(f"PASS {VECTOR_PATH.name} {hashlib.sha256(generated).hexdigest()}")
        return 0
    print(generated.decode(), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
