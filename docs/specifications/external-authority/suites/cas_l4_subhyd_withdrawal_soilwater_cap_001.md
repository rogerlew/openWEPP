---
suite_id: cas_l4_subhyd_withdrawal_soilwater_cap_001
title: WB19 Realized-Withdrawal Soil-Water Cap Suite
status: active
authority_level: 4
domain: subhyd
process_family: lateral_drain
sc_invariant_refs:
  - SC-SUBHYD-001#INV-SUBHYD-016
  - SC-WATBAL-001#INV-WATBAL-009
external_citations:
  - citation_id: EXT-SUBHYD-WB19-CH6-001
    source_type: book
    title: WEPP User Summary Chapter 6 Subsurface Hydrology and Drainage
    locator: references/50201000/chap6.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-06-01
    notes: "Lateral/drainage flux laws and daily continuity constraints."
fixtures:
  - fixture_id: FX-SUBHYD-WB19-SOILW-CAP-001
    path: tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001/withdrawal_soilwater_cap_cases.json
    fixture_class: component
    units_basis: SI
    hash: db56774abf6a274677e02654fb6c02523f589fb5f30335533ab707040d7ba8ed
    source_repo: /workdir/openWEPP
    source_commit: 2c480a547d9021654ca85da1698593fca6b2768c
    source_path: tests/fixtures/constitutive/cas_l4_subhyd_withdrawal_soilwater_cap_001/withdrawal_soilwater_cap_cases.json
    source_sha256: db56774abf6a274677e02654fb6c02523f589fb5f30335533ab707040d7ba8ed
    transform_note: "Repository-authored constitutive fixture; soil-water cap authority is captured in SC-SUBHYD-001 and SC-WATBAL-001."
tolerances:
  mode: abs
  abs:
    value: 1.0e-12
    comparator: "<="
  units: m
  notes: "Withdrawal and soil-water state assertions use strict absolute tolerance."
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

# cas_l4_subhyd_withdrawal_soilwater_cap_001 WB19 Realized-Withdrawal Soil-Water Cap Suite

## Purpose

Validate that WB19 lateral (`q`) and drainage (`Qdd`) realized withdrawals are
bounded by pre-phase `wb11_soil_water` and fail closed on over-withdrawal
instead of silently flooring negative residual storage.

## Authority Links

- Contract invariants:
  - `SC-SUBHYD-001#INV-SUBHYD-016`
  - `SC-WATBAL-001#INV-WATBAL-009`
- External citations:
  - `EXT-SUBHYD-WB19-CH6-001`

## Expected Behavior

1. In-domain withdrawal must emit `HKERNEL-WB11-LAT-OK-001` /
   `HKERNEL-WB11-DRAIN-OK-001` and preserve exact subtraction on
   `wb11_soil_water`.
2. Over-withdrawal (`q > wb11_soil_water_before` or
   `Qdd > wb11_soil_water_before`) must fail with typed domain violations
   (`HKERNEL-WB11-LAT-E-003` / `HKERNEL-WB11-DRAIN-E-003`).
3. Post-subtraction flooring/clamping behavior is forbidden in this lane.

## Fixture Coverage

1. Lateral in-domain withdrawal case.
2. Lateral over-withdrawal rejection case.
3. Drainage in-domain withdrawal case.
4. Drainage over-withdrawal rejection case.

## Tolerance Policy

- Mode: `abs`
- Absolute: `1.0e-12`
- Units: `m`

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block acceptance until WB19 soil-water cap authority is
  restored.

## Implementation Notes

- Executed by
  `tests/integration/hphys0224_wb19_withdrawal_soilwater_cap_contract.rs`.
