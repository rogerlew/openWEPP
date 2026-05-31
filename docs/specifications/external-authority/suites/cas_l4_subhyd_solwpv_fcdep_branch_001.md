---
suite_id: cas_l4_subhyd_solwpv_fcdep_branch_001
title: WB19 `solwpv` FCDEP Branch Legacy-Conformance Suite
status: active
authority_level: 5
domain: subhyd
process_family: lateral_drain
sc_invariant_refs:
  - SC-SUBHYD-001#INV-SUBHYD-015
  - SC-WATBAL-001#INV-WATBAL-009
external_citations:
  - citation_id: EXT-SUBHYD-CH6-001
    source_type: book
    title: WEPP User Summary Chapter 6 Subsurface Flow and Drainage
    locator: references/50201000/chap6.pdf
    version_or_edition: NSERL Report No. 11 (August 1995)
    retrieved_utc: 2026-05-31
    notes: "Branch law for lateral-flow coupling and saturated-depth updates."
  - citation_id: EXT-SUBHYD-BASELINE-WATBAL-001
    source_type: other
    title: WEPP baseline watbal.for branch authority
    locator: /workdir/wepp-forest_260430_baseline/src/watbal.for
    version_or_edition: dac3c950d8b16cc73774bf5ce2e7e11f80baac70
    retrieved_utc: 2026-05-31
    notes: "Static provenance: fcdep mutation is guarded by solwpv < 2006."
fixtures:
  - fixture_id: FX-SOLWPV-FCDEP-BRANCH-001
    path: tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/solwpv_fcdep_branch_cases.json
    fixture_class: component
    units_basis: m_and_dimensionless
    hash: f8f868dedcfb17c5e8862689ee21c920b151f02029bd26ad307e8c11be248163
    source_repo: /workdir/openWEPP
    source_commit: a2358ab1709adb6f78b1b4ed1b4e31c3f1c52b20
    source_path: tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/solwpv_fcdep_branch_cases.json
    source_sha256: f8f868dedcfb17c5e8862689ee21c920b151f02029bd26ad307e8c11be248163
    transform_note: "Repository-authored constitutive fixture for WB19 branch authority."
    seed_or_case: solwpv_fcdep_branch_matrix
tolerances:
  mode: abs
  abs:
    value: 1.0e-12
    comparator: "<="
  units: m_and_dimensionless
  notes: "Applies to q, watyld, fcdep, and unsdep comparisons."
gate_lane: periodic
failure_class: investigation
runtime_cost_class: component
owner: openWEPP maintainers
provenance:
  authored_by: Codex
  authored_utc: 2026-05-31
  last_updated_utc: 2026-05-31
notes: "Legacy-anchored branch-conformance suite; non-blocking investigation lane pending independent constitutive authority."
---

# cas_l4_subhyd_solwpv_fcdep_branch_001 WB19 `solwpv` FCDEP Branch Legacy-Conformance Suite

## Purpose

Enforce baseline-authoritative WB19 `solwpv` branch law for saturated-depth
mutation:

- `solwpv < 2006` permits `fcdep/unsdep` mutation via `q/watyld`.
- `solwpv >= 2006` forbids `fcdep` mutation from that term.

## Authority Links

- `SC-SUBHYD-001#INV-SUBHYD-015`
- `SC-WATBAL-001#INV-WATBAL-009`
- baseline static provenance:
  `/workdir/wepp-forest_260430_baseline/src/watbal.for`

This suite is intentionally classified as legacy-conformance evidence
(`periodic`/`investigation`) and is not a constitutive physics authority gate.

## Expected Behavior

1. Legacy modes (`solwpv < 2006`) must apply `fcdep = max(fcdep - q/watyld, 0)`.
2. `solwpv = 2006` must preserve `fcdep` under equivalent forcing.
3. Disturbed-soil modes (`solwpv >= 9001`) must preserve `fcdep` under
   equivalent forcing.
4. Coupled outputs `q`, `wb19_watyld`, `wb19_fcdep`, `wb19_unsdep` must be
   finite and within fixture tolerance.

## Fixture Coverage

1. `solwpv_2005_updates_fcdep`
2. `solwpv_2006_does_not_update_fcdep`
3. `solwpv_9002_does_not_update_fcdep`

## Gate and Failure Semantics

- Lane: `periodic`
- Failure class: `investigation`
- Failure action: record and route to governance adjudication; no default CI
  block.

## Implementation Notes

Executed by
`tests/integration/auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs`.
