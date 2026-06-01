---
suite_id: cas_l4_subhyd_layer_pool_withdrawal_cap_001
title: WB19 Layer-Pool Withdrawal Cap Authority Suite
status: active
authority_level: 4
domain: subhyd
process_family: lateral_drain
sc_invariant_refs:
  - SC-SUBHYD-001#INV-SUBHYD-017
  - SC-WATBAL-001#INV-WATBAL-009
external_citations:
  - citation_id: EXT-SUBHYD-WB19-CH6-001
    source_type: book
    title: WEPP User Summary Chapter 6 Subsurface Hydrology and Drainage
    locator: references/50201000/chap6.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-06-01
    notes: "Layer-derived drainable storage controls realized WB19 lateral/drain withdrawals."
fixtures:
  - fixture_id: FX-SUBHYD-WB19-LAYER-POOL-001
    path: tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/layer_pool_withdrawal_cap_cases.json
    fixture_class: component
    units_basis: SI
    hash: 7d7ef77f779e6dfdb84dfce7a45bd2138821b797ddd55daff1969e11ce811bd6
    source_repo: /workdir/openWEPP
    source_commit: 7833b6bf2b3412c763c0b900839c97b24897bb60
    source_path: tests/fixtures/constitutive/cas_l4_subhyd_layer_pool_withdrawal_cap_001/layer_pool_withdrawal_cap_cases.json
    source_sha256: 7d7ef77f779e6dfdb84dfce7a45bd2138821b797ddd55daff1969e11ce811bd6
    transform_note: "Repository-authored constitutive fixture to enforce WB19 layer-derived available-pool authority and reject legacy max-reconciliation."
tolerances:
  mode: abs
  abs:
    value: 1.0e-12
    comparator: "<="
  units: m
  notes: "Withdrawal and post-phase soil-water assertions use strict absolute tolerance."
gate_lane: required
failure_class: hard-fail
runtime_cost_class: component
owner: openWEPP maintainers
provenance:
  authored_by: Codex
  authored_utc: 2026-06-01
  last_updated_utc: 2026-06-01
notes: ""
---

# cas_l4_subhyd_layer_pool_withdrawal_cap_001 WB19 Layer-Pool Withdrawal Cap Authority Suite

## Purpose

Validate that WB19 lateral (`q`) and drainage (`Qdd`) withdrawals are capped by
layer-derived drainable storage from active `theta/drfc` surfaces and not
inflated by legacy reconciliation terms.

## Authority Links

- Contract invariants:
  - `SC-SUBHYD-001#INV-SUBHYD-017`
  - `SC-WATBAL-001#INV-WATBAL-009`
- External citation:
  - `EXT-SUBHYD-WB19-CH6-001`

## Expected Behavior

1. Lateral and drainage phases emit deterministic `OK` status for in-domain
   fixtures and preserve exact `wb11_soil_water` subtraction by realized
   withdrawal.
2. Raising `wb11_drainable_storage` legacy compatibility scalar alone must not
   increase realized `q` or `Qdd` under fixed layer state.
3. Production source must not contain WB19 available-pool reconciliation via
   `max(layer_pool, legacy_term)`.

## Fixture Coverage

1. Lateral low-legacy baseline.
2. Lateral high-legacy perturbation under identical layer state.
3. Drainage low-legacy baseline.
4. Drainage high-legacy perturbation under identical layer state.

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block acceptance until WB19 layer-pool authority is restored.

## Implementation Notes

- Executed by
  `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs`.
