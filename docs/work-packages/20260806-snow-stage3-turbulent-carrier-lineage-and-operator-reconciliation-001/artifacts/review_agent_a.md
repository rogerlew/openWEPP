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
