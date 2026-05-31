---
suite_id: cas_l4_soil_wp_minus1500_001
title: Soil Wilting Point Constitutive Suite (-1500 kPa)
status: active
authority_level: 4
domain: soil
process_family: fc_wp
sc_invariant_refs:
  - SC-SOIL-001#INV-SOIL-014
  - SC-SOIL-001#INV-SOIL-004
external_citations:
  - citation_id: EXT-SOIL-WP-CH7-001
    source_type: book
    title: WEPP User Summary Chapter 7 Soil Component
    locator: references/50201000/chap7.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-05-31
    notes: "Constitutive WP lineage and bounded moisture domains."
fixtures:
  - fixture_id: FX-WP-NOMINAL-001
    path: tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/nominal_case.json
    fixture_class: component
    units_basis: SI
    seed_or_case: wp_minus1500_nominal
  - fixture_id: FX-WP-BOUNDARY-001
    path: tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/boundary_case.json
    fixture_class: component
    units_basis: SI
    seed_or_case: wp_minus1500_boundary
  - fixture_id: FX-WP-INVALID-001
    path: tests/fixtures/constitutive/cas_l4_soil_wp_minus1500_001/invalid_wp_gt_fc.json
    fixture_class: component
    units_basis: SI
    seed_or_case: wp_minus1500_invalid_wp_gt_fc
tolerances:
  mode: mixed
  abs:
    value: 1.0e-9
    comparator: "<="
  rel:
    value: 1.0e-6
    comparator: "<="
  units: m3_m3 and mm
  notes: "Absolute and relative checks for constitutive domain/order behavior."
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

# cas_l4_soil_wp_minus1500_001 Soil Wilting Point Constitutive Suite (-1500 kPa)

## Purpose

Validate constitutive wilting-point (`theta_wp`) behavior as an authoritative
Level-4 physics gate for FC/WP publication lineage.

## Authority Links

- Contract invariants:
  - `SC-SOIL-001#INV-SOIL-014`
  - `SC-SOIL-001#INV-SOIL-004`
- External citations:
  - `EXT-SOIL-WP-CH7-001`

## Expected Behavior

1. Layer constitutive ordering is preserved:
   `porosity >= theta_fc >= theta_wp >= 0`.
2. Aggregate WP storage (`sum(theta_wp_i * dg_i) * 1000`) is finite and
   non-negative.
3. Invalid `theta_wp > theta_fc` states are fail-closed.

## Fixture Coverage

1. Nominal in-domain profile.
2. Boundary profile with `theta_wp == 0` in one layer.
3. Invalid profile with `theta_wp > theta_fc`.

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

- Executed by
  `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`.
