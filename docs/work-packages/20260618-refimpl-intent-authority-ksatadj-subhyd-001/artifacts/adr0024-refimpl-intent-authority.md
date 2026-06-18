# ADR-0024 Reference-Implementation Intent Authority

Package:
`20260618-refimpl-intent-authority-ksatadj-subhyd-001`

## Result

Static:

- Canonical ADR authored at
  `docs/decisions/0024-reference-implementation-intent-authority.md`.
- Decision index updated at `docs/decisions/README.md`.
- Correctness authority model updated at
  `docs/specifications/correctness-authority-model.md`.

## Decision Summary

ADR-0024 is accepted by operator direction for this package. It allows
reference-implementation source intent to serve as an `A0` provenance anchor
for empirical or conceptual process models that lack stronger external physical
authority.

The rule is narrow:

- source intent must be extracted from cited source files/routines with commit
  provenance;
- the binding authority exists only after canonical `SC-*` contract text
  encodes the invariant or obligation;
- legacy binary output remains `A6` investigation evidence only; and
- known legacy bugs, disabled branches, and non-conservation artifacts must be
  flagged as non-authoritative.

## First Application

The first concrete application is the forest disturbed-soil `ksatadj`
effective-conductivity model in `SC-SUBHYD-001`.

Static:

- Prior STAGE2-LATQCC evidence localized the remaining H2637 absolute lateral
  magnitude question to the forest `ksatadj` conductivity driver.
- ADR-0024 supplies the missing authority class for that empirical model without
  promoting legacy output magnitudes to targets.

Ran:

- Not applicable for this artifact; this is a documentation/authority decision.
