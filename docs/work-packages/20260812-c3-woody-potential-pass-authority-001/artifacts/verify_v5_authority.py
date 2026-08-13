#!/usr/bin/env python3
"""Fail-closed, implementation-independent checks for frozen V5 authority."""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
DEFINITION = HERE / "openwepp_c3_woody_v5_definition.json"
VECTORS = HERE / "openwepp_c3_woody_v5_vectors.json"
GENERATOR = HERE / "reference_calculator_v5.py"
MODEL_STACK_COPY = (
    ROOT
    / "docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts"
    / "openwepp_c3_woody_v5_definition.json"
)
V4 = (
    ROOT
    / "docs/work-packages/20260812-c3-woody-shared-state-authority-001/artifacts"
    / "openwepp_c3_woody_v4_definition.json"
)

EXPECTED = {
    DEFINITION: "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
    VECTORS: "6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d",
    GENERATOR: "4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775",
    MODEL_STACK_COPY: "0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3",
    V4: "8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437",
}

REQUIRED_POISON_KEYS = {
    "authorization_amount_as_rate",
    "authorization_as_finalized_debit",
    "borrow_unused_authorization",
    "cap_before_constitutive_law",
    "cap_tolerance_repairs_identity_or_basis",
    "continue_from_potential_candidate",
    "double_interval",
    "double_tile_fraction",
    "gas_energy_not_resolved",
    "omit_interval",
    "omit_tile_fraction",
    "partial_commit",
    "producer_supplied_zero_closure",
    "q_law_overwritten_by_q",
    "reauthorization_after_final_pass",
    "reselect_within_tie_perturbation",
    "scalar_ratio_all_layers",
    "sequential_clamp_potential_q",
    "sorted_layer_id_active_caps",
    "stale_generalized_branch",
    "stale_transaction",
    "stand_cap_used_inside_tile_law",
    "strict_less_than_tie",
    "wrong_basis",
    "wrong_layer",
    "wrong_occupancy",
    "wrong_tile_authorization",
}

RESIDUAL_IDENTITIES = [
    "sun_gas_minus_q1",
    "shade_gas_minus_q1",
    "sun_gas_minus_vulnerability_demand",
    "shade_gas_minus_vulnerability_demand",
    "q1_sum_minus_q2",
    "q2_minus_capped_q3_sum",
]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def canonical_bytes(value: object) -> bytes:
    return (
        json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n"
    ).encode("utf-8")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"FAIL: {message}")


for path, expected in EXPECTED.items():
    require(sha256(path) == expected, f"digest mismatch: {path}")

definition = json.loads(DEFINITION.read_text(encoding="utf-8"))
vectors = json.loads(VECTORS.read_text(encoding="utf-8"))

require(definition["model_version"] == "OPENWEPP_C3_WOODY_V5", "model identity")
require(definition["canonical_contract"] == "SC-VEGETATION-001@9", "contract identity")
require(
    definition["base_model_definition"]["sha256"] == EXPECTED[V4],
    "immutable V4 import",
)
require(
    definition["independent_fixture"]["sha256"] == EXPECTED[VECTORS],
    "fixture digest binding",
)
require(
    definition["independent_fixture"]["generator_sha256"] == EXPECTED[GENERATOR],
    "generator digest binding",
)
require(bool(definition["canonical_section_sha256"]), "section digest binding")
require(all(vectors["checks"].values()), "fixture structural checks")

coupled = vectors["families"]["complete_coupled_capped_solve"]
accepted = coupled["accepted_constrained_all_cap"]
layers = accepted["fluxes"]["q3"]
require(
    any(
        row["branch"] == "authorization_active_or_tie"
        and row["q_law_kg_m2_tile_s"] > 0.0
        for row in layers
    ),
    "positive cap-active layer",
)
require(len(accepted["normalized_residuals"]) == 6, "six coupled residuals")
require(
    max(abs(row["normalized"]) for row in accepted["normalized_residuals"]) <= 1.0,
    "accepted coupled residual tolerance",
)
fluxes = accepted["fluxes"]
emax = coupled["operands"]["emax"]
raw_residuals = [
    fluxes["gas_energy_transpiration_sun"] - fluxes["q1_sun"],
    fluxes["gas_energy_transpiration_shade"] - fluxes["q1_shade"],
    fluxes["gas_energy_transpiration_sun"] - emax["sun"] * fluxes["v_sun"],
    fluxes["gas_energy_transpiration_shade"] - emax["shade"] * fluxes["v_shade"],
    fluxes["q1_sun"] + fluxes["q1_shade"] - fluxes["q2"],
    fluxes["q2"] - sum(row["q_final_kg_m2_tile_s"] for row in layers),
]
v3_scale = max(
    1.0e-12,
    emax["sun"],
    emax["shade"],
    abs(fluxes["q1_sun"]),
    abs(fluxes["q1_shade"]),
    abs(fluxes["q2"]),
    *(abs(row["q_law_kg_m2_tile_s"]) for row in layers),
)
cap_scale = max(
    v3_scale,
    *(abs(row["cap_rate_kg_m2_tile_s"]) for row in layers),
    *(abs(row["q_final_kg_m2_tile_s"]) for row in layers),
)
for index, (record, raw) in enumerate(
    zip(accepted["normalized_residuals"], raw_residuals, strict=True)
):
    scale = cap_scale if index == 5 else v3_scale
    tolerance = 1.0e-12 + 1.0e-9 * scale
    require(record["raw_kg_m2_tile_s"] == raw, f"residual raw: {record['identity']}")
    require(record["scale_kg_m2_tile_s"] == scale, f"residual scale: {record['identity']}")
    require(record["tolerance"] == tolerance, f"residual tolerance: {record['identity']}")
    require(record["normalized"] == raw / tolerance, f"normalized residual: {record['identity']}")

for row in layers:
    expected_active = row["cap_rate_kg_m2_tile_s"] <= row["q_law_kg_m2_tile_s"]
    require(
        (row["branch"] == "authorization_active_or_tie") == expected_active,
        f"branch identity: {row['layer_id']}",
    )
    require(
        row["q_final_kg_m2_tile_s"]
        == min(row["q_law_kg_m2_tile_s"], row["cap_rate_kg_m2_tile_s"]),
        f"capped flux: {row['layer_id']}",
    )
    require(
        row["finalized_use_kg_m2_stand_ground"]
        <= row["authorization_kg_m2_stand_ground"],
        f"final use bound: {row['layer_id']}",
    )
    require(
        row["authorization_kg_m2_stand_ground"]
        <= row["potential_request_kg_m2_stand_ground"],
        f"authorization/request bound: {row['layer_id']}",
    )

configured_order = [row["layer_id"] for row in coupled["operands"]["layers"]]
reconstructed_active = [
    row["layer_id"]
    for row in layers
    if row["cap_rate_kg_m2_tile_s"] <= row["q_law_kg_m2_tile_s"]
]
require(
    accepted["active_water_caps"] == reconstructed_active,
    "active caps follow configured layer order",
)
require(
    [row["layer_id"] for row in layers] == configured_order,
    "accepted layer operand order",
)

closure = accepted["hydrology_owner_closure"]
for row in layers:
    layer = row["layer_id"]
    beginning = closure["beginning_inventory_kg_m2_stand_ground"][layer]
    ending = closure["ending_inventory_kg_m2_stand_ground"][layer]
    require(
        beginning - row["finalized_use_kg_m2_stand_ground"] - ending == 0.0,
        f"hydrology finalized-use debit: {layer}",
    )

fully = coupled["fully_authorized_value_reduction"]
for key, expected in fully["uncapped_reference"]["solution"].items():
    actual = fully["capped_result"]["solution"][key]
    require(abs(actual - expected) <= max(2.0e-7, 2.0e-9 * abs(expected)), f"fully authorized reduction: {key}")
fully_closure = fully["capped_result"]["hydrology_owner_closure"]
for row in fully["capped_result"]["fluxes"]["q3"]:
    layer = row["layer_id"]
    require(
        fully_closure["authorization_debit_poison_ending"][layer]
        == fully_closure["beginning_inventory_kg_m2_stand_ground"][layer]
        - row["authorization_kg_m2_stand_ground"],
        f"authorization-debit poison reconstruction: {layer}",
    )
require(
    any(
        row["authorization_kg_m2_stand_ground"]
        != row["finalized_use_kg_m2_stand_ground"]
        for row in fully["capped_result"]["fluxes"]["q3"]
    ),
    "authorization operand is numerically distinct from finalized-use debit",
)
alternate = coupled["alternate_warm_start"]["result"]
for key, expected in accepted["solution"].items():
    actual = alternate["solution"][key]
    require(abs(actual - expected) <= max(2.0e-7, 2.0e-9 * abs(expected)), f"alternate warm start: {key}")

law_rows = vectors["families"]["controlled_law_branch_nonacceptance"]
require(law_rows["acceptance_evidence"] is False, "controlled law branch posture")
require(
    all(
        row["branch"] == "constitutive_law"
        and row["q_final_kg_m2_tile_s"] == row["q_law_kg_m2_tile_s"]
        for row in law_rows["layers"]
    ),
    "controlled law branch",
)
tie = vectors["families"]["exact_and_near_tie"]["cases"]
require(tie[1]["case"] == "exact_tie", "exact tie position")
require(tie[1]["branch"] == "authorization_active_or_tie", "tie branch")
require(tie[1]["dq_final_d_root_potential"] == 0.0, "tie derivative")
require(tie[0]["cap_rate_kg_m2_tile_s"] < tie[0]["q_law_kg_m2_tile_s"], "near-tie cap side")
require(tie[2]["q_law_kg_m2_tile_s"] < tie[2]["cap_rate_kg_m2_tile_s"], "near-tie law side")

jacobian = coupled["executed_coupled_exact_tie_jacobian"]
require(jacobian["unperturbed"]["branch"] == "authorization_active_or_tie", "coupled tie branch")
require(jacobian["frozen_cap_derivative"] == 0.0, "frozen tie derivative")
require(jacobian["frozen_plus"]["q_final_kg_m2_tile_s"] == jacobian["frozen_minus"]["q_final_kg_m2_tile_s"], "frozen perturbation pair")
require(jacobian["trial_plus_reselected"]["branch"] != jacobian["trial_minus_reselected"]["branch"], "trial branch reselection")

failures = vectors["families"]["capped_failures"]
require(
    {row["failure"] for row in failures}
    == {"singular", "iteration_limit", "backtracking_limit", "authorization_domain", "hydraulic_redistribution:soil-2"},
    "executed failure family",
)
for row in failures:
    require(row["candidate"] is None, f"failure candidate: {row['failure']}")
    require(row["rollback_byte_identical"], f"failure rollback: {row['failure']}")
    require(
        row["active_water_caps"] == row["diagnostics"]["active_water_caps"],
        f"failure active-cap diagnostics: {row['failure']}",
    )
    diagnostics = row["diagnostics"]
    for field in ("model_definition_sha256", "transaction_id", "occupancy_id", "pass", "solve", "iterations", "residual_norms", "step_norm", "backtracking_count", "active_bounds", "active_water_caps", "bracket", "pivot_magnitude", "matrix_norm"):
        require(field in diagnostics, f"failure diagnostic {field}: {row['failure']}")
    layer_operands = diagnostics["layer_operands_in_configuration_order"]
    residuals = diagnostics["residual_norms"]
    if row["failure"] == "authorization_domain":
        require(not layer_operands and not residuals, "authorization fails before evaluation")
        continue
    if row["failure"].startswith("hydraulic_redistribution:"):
        require(not residuals and not row["active_water_caps"], "redistribution rejects before residual assembly")
        require(layer_operands[-1]["q_law_kg_m2_tile_s"] < 0.0, "redistribution negative law flux")
        require(layer_operands[-1]["rejected_before_complementarity"], "redistribution pre-complementarity rejection")
        continue
    require(
        [item["layer_id"] for item in layer_operands] == configured_order,
        f"failure configured layer order: {row['failure']}",
    )
    derived_active = []
    for item in layer_operands:
        is_active = item["cap_rate_kg_m2_tile_s"] <= item["q_law_kg_m2_tile_s"]
        require(
            (item["branch"] == "authorization_active_or_tie") == is_active,
            f"failure tie-rule branch: {row['failure']}:{item['layer_id']}",
        )
        require(
            item["q_final_kg_m2_tile_s"]
            == min(item["q_law_kg_m2_tile_s"], item["cap_rate_kg_m2_tile_s"]),
            f"failure capped flux: {row['failure']}:{item['layer_id']}",
        )
        require(
            item["generalized_derivative_branch"]
            == ("zero" if is_active else "constitutive_law"),
            f"failure generalized branch: {row['failure']}:{item['layer_id']}",
        )
        if is_active:
            derived_active.append(item["layer_id"])
    require(
        diagnostics["active_water_caps"] == derived_active,
        f"failure active caps derived from failed iterate: {row['failure']}",
    )
    require(
        [record["identity"] for record in residuals] == RESIDUAL_IDENTITIES,
        f"typed ordered failure residuals: {row['failure']}",
    )
    for record in residuals:
        expected_tolerance = 1.0e-12 + 1.0e-9 * record["scale_kg_m2_tile_s"]
        require(
            record["tolerance"] == expected_tolerance,
            f"failure residual tolerance: {row['failure']}:{record['identity']}",
        )
        require(
            record["normalized"]
            == record["raw_kg_m2_tile_s"] / record["tolerance"],
            f"failure normalized residual: {row['failure']}:{record['identity']}",
        )

migration = vectors["families"]["v4_to_v5_identity_rebind"]
source_preimage = migration["complete_v4_source_preimage"]
payload = {
    key: value
    for key, value in source_preimage.items()
    if key not in {"model_definition_sha256", "configuration_sha256"}
}
payload_bytes = canonical_bytes(payload)
require(sha256_bytes(payload_bytes) == migration["payload_sha256_before"], "complete V4 payload digest")
require(migration["payload_sha256_before"] == migration["payload_sha256_after"], "V4-to-V5 payload preservation")
actual_v5_model = EXPECTED[DEFINITION]
v5_configuration = sha256_bytes(
    b"OPENWEPP_V5_CONFIG_IDENTITY_V1\0"
    + actual_v5_model.encode("ascii")
    + b"\0"
    + source_preimage["configuration_sha256"].encode("ascii")
)
migrated_preimage = dict(payload)
migrated_preimage["model_definition_sha256"] = actual_v5_model
migrated_preimage["configuration_sha256"] = v5_configuration
v4_calculator_path = V4.parent / "reference_calculator.py"
spec = importlib.util.spec_from_file_location("openwepp_v4_authority", v4_calculator_path)
require(spec is not None and spec.loader is not None, "load V4 canonical encoder")
v4_calculator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v4_calculator)
v5_state = v4_calculator.whole_state_preimage_digest(migrated_preimage)
require(v5_configuration != migration["v4_identity"]["configuration_sha256"], "distinct V5 configuration identity")
require(v5_state != migration["v4_identity"]["state_sha256"], "distinct V5 state identity")
print(f"v5_configuration_sha256={v5_configuration}")
print(f"v5_state_sha256={v5_state}")

for row in vectors["families"]["phase_rollback_injections"]:
    require(row["candidate_was_materially_mutated"], f"rollback mutation: {row['phase']}")
    require(not row["candidate_survives"], f"rollback candidate: {row['phase']}")
    require(row["beginning_owner_bytes_before_sha256"] == row["beginning_owner_bytes_after_sha256"], f"rollback bytes: {row['phase']}")
require(
    {row["mutated_owner_surface"] for row in vectors["families"]["phase_rollback_injections"]}
    == {"vegetation", "water", "biogeochemistry", "energy", "transaction", "diagnostics"},
    "all rollback owner surfaces",
)

require(set(vectors["poisons"]) == REQUIRED_POISON_KEYS, "exact required poison inventory")
for name, row in vectors["poisons"].items():
    require(row["executed"], f"poison executed: {name}")
    if "typed_error" in row:
        require(row["candidate"] is None and bool(row["typed_error"]), f"typed poison: {name}")
    else:
        require(row["discriminates"], f"poison discrimination flag: {name}")
        require(row.get("accepted") != row.get("rejected"), f"numeric poison discrimination: {name}")

zero_rows = vectors["families"]["zero_dry_frozen_inaccessible"]
require(
    [row["layer_id"] for row in zero_rows]
    == ["dry-layer", "frozen-layer", "inaccessible-layer", "zero-root-layer"],
    "distinct unavailable and zero-root cases",
)
for row in zero_rows:
    require(
        row["q_law_kg_m2_tile_s"] == 0.0
        and row["q_final_kg_m2_tile_s"] == 0.0
        and row["finalized_use_kg_m2_stand_ground"] == 0.0,
        f"exact zero layer: {row['layer_id']}",
    )
zero_root = zero_rows[-1]
require(
    zero_root.get("accessible") is True
    and zero_root.get("dry") is False
    and zero_root.get("frozen") is False
    and zero_root.get("root_fraction") == 0.0,
    "accessible non-dry non-frozen zero-root cause",
)

print("PASS: V5 capped-pass authority and independent vectors are frozen and consistent")
