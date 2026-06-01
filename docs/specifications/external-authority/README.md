# External-Authority Suites

Status: Active
Last updated: 2026-05-31
Scope: suite structure and authoring standards for external-authority suites
used by openWEPP correctness gates

## Purpose

Define where external-authority suites live, their minimum shape, and standard
practices for linking them to canonical `SC-*` invariants.

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
  - `suite-schema.md` (normative required-field schema)
  - `suite-template.md` (suite-authoring template)
  - `required-suite-obligations.json` (machine-checked anchor obligations)
  - `promotion-protocol.md` (lane/failure posture change protocol)
  - `registry-template.yaml` (registry structure template)
  - `suites/` (suite definitions; one file per suite ID)
  - `registry.*` (suite registry index, introduced/maintained by follow-on
    packages)

### Test/fixture side

- `tests/fixtures/constitutive/<suite_id>/...`
- `tests/integration/<suite_id>_contract.rs` (or equivalent contract-derived
  harness naming)

Fixture root guidance:
- `tests/fixtures/constitutive/README.md`

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

See:
- `docs/specifications/external-authority/suite-schema.md`
- `docs/specifications/external-authority/suite-template.md`

## Standard Practices (Normative)

1. Test laws, not parity numbers.
2. Keep units explicit at all boundaries.
3. Treat missing citation or missing invariant linkage as non-compliant.
4. Fail closed on invalid/missing required suite inputs.
5. Preserve deterministic fixture provenance:
   - require per-fixture `sha256`,
   - require lockfile `fixtures.sha256`,
   - require provenance sidecar `fixtures.provenance.yaml` with source commit
     and source hash.
6. Level-3 legacy/sanity suites may encode legacy-anchored branch laws, but
   they must remain non-blocking investigation evidence; Level-4+ constitutive
   acceptance criteria must not depend on legacy parity matching.
7. Required-case anchor obligations for guarded suites must be encoded in
   `required-suite-obligations.json` and enforced by contract-derived tests.
8. Lane/failure posture changes must follow `promotion-protocol.md` and must
   not remove anchor cases, loosen thresholds, or shrink case cardinality.
9. Any Level-4 suite running non-blocking (`periodic`/`investigation`) must
   carry an explicit queued/in-progress closure follow-on package reference in
   `required-suite-obligations.json` and `docs/work-packages/README.md`.

## Package Sequencing

When external-authority suite work affects kernel/runtime acceptance:

1. amend canonical contract authority (`SC-*`),
2. add or amend contract-derived suite tests,
3. record pre-implementation gate evidence, then
4. modify production code if required.
