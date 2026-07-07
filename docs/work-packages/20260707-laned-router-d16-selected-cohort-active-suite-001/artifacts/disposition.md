# Disposition

Status: EXECUTED-HOLD-ACTIVE-RUN. Evidence mode: Static + Ran.

## Outcome

The package scaffolded and executed the selected-cohort active suite until the
first hard active-run blocker.

Closed within this package:

- Package-local selected cohort materialization from source-authorized inputs.
- Corrected mode-specific runfiles with separate output dirs.
- Corrected H2637 active plain vs true active hybrid timing and publication
  delta evidence.

Not closed:

- Selected-cohort active plain-vs-hybrid suite completion.
- D16/default hybrid promotion.

## Hold

`mn_corn_h4` active plain fails closed on the Rev-21 Lane D operand guard:
positive LAI with missing/non-positive typed `canhgt` at lane 1 day 136.

This is a legitimate active-run hold and must not be bypassed by defaulting or
inventing canopy height. The first follow-on is an authority-backed row-crop
canopy-height runtime publication/source-lift.

## Status Mapping

Package closure outcome:

- `EXECUTED-HOLD-ACTIVE-RUN`

D16 promotion posture:

- Still blocked. Even the completed H2637 pair retains publication deltas, and
  the selected cohort cannot yet run to completion.
