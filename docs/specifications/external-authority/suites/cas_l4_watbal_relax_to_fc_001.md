---
suite_id: cas_l4_watbal_relax_to_fc_001
title: WB18 Relax-to-FC and Near-FC Dp Cutoff Suite
status: active
authority_level: 4
domain: watbal
process_family: percolation
sc_invariant_refs:
  - SC-WATBAL-001#INV-WATBAL-006
external_citations:
  - citation_id: EXT-WATBAL-PERC-CH5-001
    source_type: book
    title: WEPP User Summary Chapter 5 Water Balance
    locator: references/50201000/chap5.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-05-31
    notes: "Percolation eligibility and FC-threshold behavior."
fixtures:
  - fixture_id: FX-RELAX-NOMINAL-001
    path: tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/near_fc_cutoff.json
    fixture_class: component
    units_basis: SI
    hash: 435555aab26fff337834c2da17383a4717a5378b0cfd18a511305f1f02ad0eee
    source_repo: /workdir/openWEPP
    source_commit: 6530baae39fdb81f6558ab0ea9a2f1d7eb01ff08
    source_path: tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/near_fc_cutoff.json
    source_sha256: 435555aab26fff337834c2da17383a4717a5378b0cfd18a511305f1f02ad0eee
    transform_note: "Repository-authored constitutive fixture; law authority in SC-WATBAL-001 and cited chapter references."
    seed_or_case: relax_near_fc_cutoff
  - fixture_id: FX-RELAX-ABOVEFC-001
    path: tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/above_fc_positive.json
    fixture_class: component
    units_basis: SI
    hash: 6b88bc3538234c7a20e4a5db2144c84ef545c9a9a43d23a2a9944f7be68e5942
    source_repo: /workdir/openWEPP
    source_commit: 6530baae39fdb81f6558ab0ea9a2f1d7eb01ff08
    source_path: tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/above_fc_positive.json
    source_sha256: 6b88bc3538234c7a20e4a5db2144c84ef545c9a9a43d23a2a9944f7be68e5942
    transform_note: "Repository-authored constitutive fixture; law authority in SC-WATBAL-001 and cited chapter references."
    seed_or_case: relax_above_fc_positive
  - fixture_id: FX-RELAX-INVALID-001
    path: tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/invalid_missing_theta.json
    fixture_class: component
    units_basis: SI
    hash: 733b5261e76dbc2e4bc4b0e9dfbfdf659e3a2bb088ddb3f2f06a11a225b54aa2
    source_repo: /workdir/openWEPP
    source_commit: 6530baae39fdb81f6558ab0ea9a2f1d7eb01ff08
    source_path: tests/fixtures/constitutive/cas_l4_watbal_relax_to_fc_001/invalid_missing_theta.json
    source_sha256: 733b5261e76dbc2e4bc4b0e9dfbfdf659e3a2bb088ddb3f2f06a11a225b54aa2
    transform_note: "Repository-authored constitutive fixture; law authority in SC-WATBAL-001 and cited chapter references."
    seed_or_case: relax_invalid_missing_theta
tolerances:
  mode: abs
  abs:
    value: 1.0e-12
    comparator: "<="
  units: m
  notes: "Near-FC cutoff assertions use strict absolute tolerance."
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

# cas_l4_watbal_relax_to_fc_001 WB18 Relax-to-FC and Near-FC Dp Cutoff Suite

## Purpose

Validate per-layer percolation cutoff behavior near field capacity and positive
deep-percolation behavior above field capacity as a blocking Level-4 gate.

## Authority Links

- Contract invariants:
  - `SC-WATBAL-001#INV-WATBAL-006`
- External citations:
  - `EXT-WATBAL-PERC-CH5-001`

## Expected Behavior

1. `theta <= fc` must produce `pei = 0` and no deep-percolation loss.
2. `theta > fc` must produce positive `pei` and non-negative `D`.
3. Missing/non-finite/domain-invalid constitutive symbols are fail-closed.

## Fixture Coverage

1. Near-cutoff (`theta == fc`) no-percolation case.
2. Above-FC positive-percolation case.
3. Missing-symbol fail-closed case.

## Tolerance Policy

- Mode: `abs`
- Absolute: `1.0e-12`
- Units: `m`

## Gate and Failure Semantics

- Lane: `required`
- Failure class: `hard-fail`
- Failure action: block acceptance until percolation eligibility closure is
  restored.

## Implementation Notes

- Executed by
  `tests/integration/auth05_level4_constitutive_authority_hardening_contract.rs`.
