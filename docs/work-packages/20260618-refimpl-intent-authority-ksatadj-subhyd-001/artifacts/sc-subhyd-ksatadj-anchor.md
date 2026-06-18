# `SC-SUBHYD-001` `ksatadj` Anchor

Package:
`20260618-refimpl-intent-authority-ksatadj-subhyd-001`

## Canonical Contract Edits

Static:

- Updated `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`.
- Added authority anchor `REF-SUBHYD-KSATADJ-INTENT`.
- Added invariant `INV-SUBHYD-032`.
- Added REFINTENT001 addendum and binding-exposure row.
- Updated the science-contract registry row for `SC-SUBHYD-001`.

## Anchor Text Summary

`REF-SUBHYD-KSATADJ-INTENT` cites:

- `input.for:467-473,592-623,748-928`
- `infpar.for:237-260,286-296,606-648`

Contract use:

- policy input,
- top-two tillage-layer saturation/FC/WP averages,
- 9001, 9002+, and 9003 effective-conductivity formulas,
- `mm h^-1` to `m s^-1` conversion, and
- source-intent guard posture.

## Invariant Summary

`INV-SUBHYD-032` requires `ksatadj = 1` conductivity to be formed from the
ADR-0024 source-intent algorithm. It specifically binds the saturation fraction
to top-two tillage-layer total water over averaged porosity and rock correction,
not to a legacy output magnitude or comparator target.

## Review Status

Static:

- The canonical contract remains `status: in_review`.
- This package did not explicitly authorize delegated independent reviewers.
- Therefore the amendment is authored and ready for independent contract review,
  but this artifact does not claim the dual independent review/verification gate
  has completed.

Ran:

- Lint evidence is recorded in `refintent_disposition.md`.
