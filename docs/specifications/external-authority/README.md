# External-Authority Constitutive Suites

Status: Active
Last updated: 2026-05-31
Scope: suite structure and authoring standards for external constitutive
authority used by openWEPP correctness gates

## Purpose

Define where external-authority constitutive suites live, their minimum shape,
and standard practices for linking them to canonical `SC-*` invariants.

This document is a framework entrypoint. AUTH02 expands it with concrete schema
templates and registry files.

## Authority Relationship

External-authority suites do not replace canonical contracts. They are
executable evidence surfaces mapped to `SC-*` invariants under the authority
model in:

- `docs/specifications/correctness-authority-model.md`

## Location and Structure (Normative)

### Specification side

- `docs/specifications/external-authority/`
  - `README.md` (this file)
  - `suites/` (suite definitions; one file per suite ID)
  - `registry.*` (suite registry index, introduced/maintained by follow-on
    packages)

### Test/fixture side

- `tests/fixtures/constitutive/<suite_id>/...`
- `tests/integration/<suite_id>_contract.rs` (or equivalent contract-derived
  harness naming)

## Minimum Suite Shape (Normative)

Each suite definition must include:

1. Suite metadata (`suite_id`, `authority_level`, `domain`).
2. Linked contract invariants (`SC-*#INV-*` references).
3. External citations with provenance/version details.
4. Fixture manifest and units declaration.
5. Tolerances and pass/fail thresholds.
6. Gate lane class (`required`, `periodic`, `manual`).
7. Failure class (`hard-fail`, `investigation`).
8. Runtime cost class (`unit`, `component`, `integration`).

## Standard Practices (Normative)

1. Test laws, not parity numbers.
2. Keep units explicit at all boundaries.
3. Treat missing citation or missing invariant linkage as non-compliant.
4. Fail closed on invalid/missing required suite inputs.
5. Preserve deterministic fixture provenance (versioned source, transform
   notes, and hash where practical).
6. Keep legacy comparator references out of constitutive acceptance criteria.

## Package Sequencing

When constitutive suite work affects kernel/runtime acceptance:

1. amend canonical contract authority (`SC-*`),
2. add or amend contract-derived suite tests,
3. record pre-implementation gate evidence, then
4. modify production code if required.

