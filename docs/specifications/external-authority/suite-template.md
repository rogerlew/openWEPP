# External Authority Suite Template

Status: Active
Last updated: 2026-05-31
Scope: authoring template for suite definitions under `suites/`

Use this template to define a new suite file:
`docs/specifications/external-authority/suites/<suite_id>.md`.

---
suite_id: cas_l4_example_domain_example_law_001
title: Example Constitutive Law Suite
status: draft
authority_level: 4
domain: example_domain
process_family: example_family
sc_invariant_refs:
  - SC-EXAMPLE-001#INV-EXAMPLE-001
external_citations:
  - citation_id: EXT-001
    source_type: paper
    title: Example Source
    locator: https://example.org/paper
    version_or_edition: v1
    retrieved_utc: 2026-05-31
fixtures:
  - fixture_id: FX-001
    path: tests/fixtures/constitutive/cas_l4_example_domain_example_law_001/case_a.json
    fixture_class: component
    units_basis: SI
    hash: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
    source_repo: /workdir/openWEPP
    source_commit: <commit-sha>
    source_path: tests/fixtures/constitutive/cas_l4_example_domain_example_law_001/case_a.json
    source_sha256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
    transform_note: "Repository-authored fixture; law authority captured in SC/external citations."
tolerances:
  mode: mixed
  abs:
    value: 1.0e-6
    comparator: "<="
  rel:
    value: 1.0e-4
    comparator: "<="
  units: mm
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

`authority_level` may be `3` (legacy/sanity), `4` (constitutive), `5`
(measured/system), or `6` (independent solver).

# <suite_id> <title>

## Purpose

State the constitutive law and what correctness claim it adjudicates.

## Authority Links

- Contract invariants:
  - `SC-...#INV-...`
- External citations:
  - `EXT-...`

## Expected Behavior

Describe law-driven expectations in executable terms (units, sign, bounds,
branch behavior).

## Fixture Coverage

List fixture classes and coverage intent:

1. Nominal in-domain case(s)
2. Boundary/near-threshold case(s)
3. Invalid-domain fail-closed case(s)

## Tolerance Policy

Restate tolerance mode and thresholds with units context.

## Gate and Failure Semantics

- Lane: `required|periodic|manual`
- Failure class: `hard-fail|investigation`
- Default adjudication action on failure.

## Implementation Notes

Optional links to package artifacts or follow-on queues.
