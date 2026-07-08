# Worker Handoff

Status: final handoff.

## Next Package

M-T2B may proceed after this package closes, but it must treat
coefficient-absent legacy cropland as legacy/off unless a later bridge contract
ratifies all five Lane D route coefficients.

Post-closure consensus supersedes the sidecar-bridge direction: do not add new
runfile/disturbed-class sidecars for coefficient authority. Reduce operator
error by making `ow-lanuse-1` the canonical production datver for new physics
and keeping legacy datvers on compatibility/legacy-driver paths.

## Binding Rules

- Keep explicit native `routing_coefficients` and authorized explicit producer
  fields as the only accepted static Lane D coefficient authority.
- Prefer producer-side materialization into native `ow-lanuse-1` `.man` files:
  WEPPpy should resolve Disturbed/native class route coefficients and write the
  five explicit `routing_coefficients` values.
- Preserve no-coefficient legacy/off fallback.
- Preserve mixed-authority fail-closed behavior.
- Do not implement hidden projection from `rrc`, `rrough`, row/rill geometry,
  cover/residue/canopy-cover fields, aggregate friction factors, erosion
  delivery ratios, or diagnostics.
- Do not introduce optional sidecars whose presence/absence changes routing
  physics for the same legacy `.man`.
- Treat legacy datvers as compatibility inputs for legacy single/MOFE drivers;
  new active/default Lane D production eligibility should require native
  `ow-lanuse-1` coefficient authority.
- Current Rust comments/errors still name native `routing_coefficients`
  because no implementation broadening occurred in this package. If a future
  package makes another explicit producer user-visible in runtime diagnostics,
  update the runtime wording and tests to say source-authorized route
  coefficients without weakening the fail-closed behavior.
- Distinguish generated zero, disabled process, missing authority, and
  explicit-disable/rollback in any groundwater/baseflow publication work.

## Reopening Route

If production needs coefficient-complete activation for current legacy cropland
managements, scaffold an `ow-lanuse-1` canonicalization/migration package rather
than a sidecar package. First actions should be:

1. Ratify `ow-lanuse-1` as the canonical production datver for new physics.
2. Define legacy datvers as compatibility/validation/rollback surfaces.
3. Update WEPPpy producers to emit native management files with embedded route
   coefficients.
4. Add openWEPP guards proving legacy datvers remain legacy/off unless
   converted and native mixed authority still fails closed.
