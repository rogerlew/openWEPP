---
suite_id: cas_l4_soil_fc_minus33_001
title: Soil Field Capacity Constitutive Suite (-33 kPa)
status: active
authority_level: 4
domain: soil
process_family: fc_wp
sc_invariant_refs:
  - SC-SOIL-001#INV-SOIL-014
  - SC-SOIL-001#INV-SOIL-004
external_citations:
  - citation_id: EXT-SOIL-FC-CH7-001
    source_type: book
    title: WEPP User Summary Chapter 7 Soil Component
    locator: references/50201000/chap7.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-05-31
    notes: "Constitutive FC/WP lineage and layer water-content bounds."
  - citation_id: EXT-SOIL-FC-LEGACY-001
    source_type: solver_doc
    title: Legacy WEPP baseline constitutive lineage
    locator: /workdir/wepp-forest_260430_baseline/src/watbal.for
    version_or_edition: dac3c950d8b16cc73774bf5ce2e7e11f80baac70
    retrieved_utc: 2026-05-31
    notes: "Baseline FC threshold usage in percolation/state publication lineage."
fixtures:
  - fixture_id: FX-FC-NOMINAL-001
    path: tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/nominal_case.json
    fixture_class: component
    units_basis: SI
    seed_or_case: fc_minus33_nominal
  - fixture_id: FX-FC-BOUNDARY-001
    path: tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/boundary_case.json
    fixture_class: component
    units_basis: SI
    seed_or_case: fc_minus33_boundary
  - fixture_id: FX-FC-INVALID-001
    path: tests/fixtures/constitutive/cas_l4_soil_fc_minus33_001/invalid_missing_theta_fc.json
    fixture_class: component
    units_basis: SI
    seed_or_case: fc_minus33_invalid_missing_theta_fc
tolerances:
  mode: mixed
  abs:
    value: 1.0e-9
    comparator: "<="
  rel:
    value: 1.0e-6
    comparator: "<="
  units: m3_m3 and mm
  notes: "Absolute check for equality/bounds, relative check for aggregate storage consistency."
gate_lane: required
failure_class: hard-fail
runtime_cost_class: component
owner: openWEPP maintainers
provenance:
  authored_by: Codex
  authored_utc: 2026-05-31
  last_updated_utc: 2026-05-31
notes: ""
---

# cas_l4_soil_fc_minus33_001 Soil Field Capacity Constitutive Suite (-33 kPa)

## Purpose

Validate constitutive field-capacity (`theta_fc`) behavior as an authoritative
Level-4 physics gate for FC/WP publication lineage.

## Authority Links

- Contract invariants:
  - `SC-SOIL-001#INV-SOIL-014`
  - `SC-SOIL-001#INV-SOIL-004`
- External citations:
  - `EXT-SOIL-FC-CH7-001`
  - `EXT-SOIL-FC-LEGACY-001`

## Expected Behavior

1. Layer constitutive ordering is preserved:
   `porosity >= theta_fc >= theta_wp >= 0`.
2. Aggregate FC storage (`sum(theta_fc_i * dg_i) * 1000`) is finite and
   non-negative.
3. Missing/invalid `theta_fc` symbols are fail-closed.

## Fixture Coverage

1. Nominal in-domain profile.
2. Boundary profile with `theta_fc == theta_wp` in one layer.
3. Invalid profile missing `theta_fc` payload.

## Tolerance Policy

- Mode: `mixed`
- Absolute: `1.0e-9`
- Relative: `1.0e-6`
- Units: `m3_m3` and `mm`

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block acceptance until constitutive ordering/guard posture is
  restored.

## Implementation Notes

- Executed by `tests/integration/auth03_level4_constitutive_gate_contract.rs`.
