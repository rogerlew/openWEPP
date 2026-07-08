# Implementation Notes

Status: `COMPLETE`
Evidence mode: Static.

Implemented in
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.

Changes:

- Added private `StepCelerity { max_celerity, max_cell_index }`.
- Changed `prepare_step_alpha()` to retain the first cell with the maximum
  wet-cell true celerity while it already fills `scratch.celerity`.
- Replaced the post-`dt` wet-cell Courant scan with
  `step_max_courant = max_celerity * dt / dx` after breakpoint clipping.
  Because `dt / dx` is a positive scalar for accepted steps, this is the same
  maximum and first-max index as the removed scan.
- Preserved fail-closed behavior for non-finite or `Cr > 1` step Courant.
- Moved additive-path `self.slope.sqrt()` below the pure-skin branch return so
  pure-skin cells do not pay additive-friction prework.
- Added
  `post_tier1_prepare_step_alpha_retains_scan_max_celerity()` to compare the
  retained summary against an explicit scratch scan and tie-first index.

Deferred/classified:

- No `Re^0.45` approximation was implemented.
- Source-free/homogeneous step specialization and static per-cell precompute
  were not implemented in this package; they remain candidates only if a later
  package proves they are behavior-preserving inside the explicit scheme.
