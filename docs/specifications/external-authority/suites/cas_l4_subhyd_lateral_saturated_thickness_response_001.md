---
suite_id: cas_l4_subhyd_lateral_saturated_thickness_response_001
title: WB19 Lateral Saturated-Thickness Response Suite
status: active
authority_level: 4
domain: subhyd
process_family: lateral_drain
sc_invariant_refs:
  - SC-SUBHYD-001#INV-SUBHYD-018
  - SC-WATBAL-001#INV-WATBAL-009
external_citations:
  - citation_id: EXT-SUBHYD-WB19-CH6-001
    source_type: book
    title: WEPP User Summary Chapter 6 Subsurface Hydrology and Drainage
    locator: references/50201000/chap6.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-06-01
    notes: "Lateral response increases with increased saturated thickness when other drivers are held fixed."
fixtures:
  - fixture_id: FX-SUBHYD-WB19-SATTHK-RESP-001
    path: tests/fixtures/constitutive/cas_l4_subhyd_lateral_saturated_thickness_response_001/lateral_saturated_thickness_response_cases.json
    fixture_class: component
    units_basis: SI
    hash: 66cf8af26c2d0f93f048bfee2abffc02a152f56767546463064a794351b793dc
    source_repo: /workdir/openWEPP
    source_commit: f01c94e86fda7829cf488c1943036210843f10b8
    source_path: tests/fixtures/constitutive/cas_l4_subhyd_lateral_saturated_thickness_response_001/lateral_saturated_thickness_response_cases.json
    source_sha256: 66cf8af26c2d0f93f048bfee2abffc02a152f56767546463064a794351b793dc
    transform_note: "Repository-authored behavioral fixture for WB19 lateral saturated-thickness response authority."
tolerances:
  mode: abs
  abs:
    value: 1.0e-12
    comparator: "<="
  units: m
  notes: "Flux comparisons and guard checks use strict absolute tolerance."
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

# cas_l4_subhyd_lateral_saturated_thickness_response_001 WB19 Lateral Saturated-Thickness Response Suite

## Purpose

Validate that WB19 lateral flux (`q`) increases when saturated thickness
increases under fixed geometry, conductivity, and forcing.

## Authority Links

- Contract invariants:
  - `SC-SUBHYD-001#INV-SUBHYD-018`
  - `SC-WATBAL-001#INV-WATBAL-009`
- External citation:
  - `EXT-SUBHYD-WB19-CH6-001`

## Expected Behavior

1. Both constitutive cases produce deterministic WB19 lateral success status.
2. The higher-saturated-thickness case yields greater realized lateral flux
   (`q_high > q_low`) by at least fixture threshold.
3. With high anisotropy forcing, realized `q` remains bounded by layer-derived
   available pool in both cases.

## Fixture Coverage

1. Low-saturated-thickness WB19 lateral case.
2. High-saturated-thickness WB19 lateral case with all other drivers fixed.

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block acceptance until WB19 lateral saturated-thickness
  response authority is restored.

## Implementation Notes

- Executed by
  `tests/integration/hphys0226_wb19_lateral_saturated_thickness_response_contract.rs`.
