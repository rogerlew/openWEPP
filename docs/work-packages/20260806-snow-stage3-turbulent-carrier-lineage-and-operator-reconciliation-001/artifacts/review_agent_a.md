# Review Agent A

Status: `HOLD at scaffold commit; amendment re-review queued`.

Evidence class: `Static` at exact clean commit
`30e843d4116411520cf9eeb7f08a3bf1ce853b78`.

Reviewer: `operator_protocol_science_review`.

## Findings

1. `CRITICAL`: raw source equality hid different first effective control
   volumes. Sequential aligns an active layer before its first solve; same-state
   uses all layers. Required projection fingerprints/state and a separate
   `INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE` class.
2. `CRITICAL`: hourly primitive means cannot reconstruct nonlinear
   Monin-Obukhov products. Required exact duration-tagged substep tuples.
3. `HIGH`: cohort/predecessor freeze omitted hashes, windows, commands,
   tolerances, and the exact `+170.2536 MJ m^-2` estimand.
4. `HIGH`: field lineage was incomplete and confused aerodynamic `z_0,aero`
   with thermal active depth. Required all radiation, turbulence, advected, and
   state fields.
5. `HIGH`: joins, partial support, aggregation, and decision predicates were
   underdetermined.
6. `MEDIUM`: zero-support applicability and mass/cold endpoint closure were
   not frozen.

Disposition at reviewed commit: `HOLD`.

The full finding text was delivered to the orchestrator and is dispositioned
in `review-disposition.md`. No model lane ran and the reviewer edited no file.

## Amended Re-review

Static re-review at clean `6dd69f8fd4f1157da633eaf03f525e389612d2ca`
remained `HOLD` with five residuals:

1. `CRITICAL`: predecessor positive energy includes active conduction while the
   comparable external carrier excludes it; required an explicit legacy bridge
   and a like-for-like external sign predicate.
2. `HIGH`: total cold closure used active cold change without lower-volume
   cancellation.
3. `HIGH`: albedo fallback and geometry-fingerprint lineage were factually
   incorrect.
4. `HIGH`: omitted-support materiality lacked a complete numerator/denominator.
5. `MEDIUM`: neutral zero-iteration and post-melt after-surface applicability
   were incomplete.

No result-bearing lane ran and the reviewer edited no file.

## Third Re-review

Static re-review at clean `317bcd0e34617b4d44e5a0912d7e23da6d4d803d`
remained `HOLD` with two residuals:

1. `HIGH`: the evolution class did not require the ordered frozen-reference to
   sequential step itself to cross sign, so projection could be misattributed.
2. `MEDIUM`: the daily frozen-active reference omitted selected snow-albedo
   value and state/fallback lineage.

All second-review residuals were otherwise closed. No result-bearing lane ran
and the reviewer edited no file.
