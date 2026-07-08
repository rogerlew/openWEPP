# Decision Lock Audit

Evidence class: Static.
Status: complete.

## Source Decision

M-T2P closed as `EXECUTED-HOLD-PROJECTION-AUTHORITY`. It rejected implicit
projection from legacy cropland fields to Lane D static route coefficients and
recorded the preferred route: make `ow-lanuse-1` the canonical native production
datver for new openWEPP physics.

## Locked Requirements

- No coefficient projection from legacy cropland fields.
- No optional route-coefficient sidecar whose presence or absence changes
  routing physics for the same `.man`.
- `ow-lanuse-1` and later ratified native datvers are canonical for new
  openWEPP production physics.
- Lane D active/default production requires every scheduled native lane to have
  a complete embedded `routing_coefficients` block.
- Pre-`ow-lanuse-1` datvers remain deprecated compatibility, validation,
  rollback, and regression-diagnosis inputs.
- All-legacy scheduled runs remain on the legacy/off path.
- Native missing coefficients, mixed native/legacy scheduled datvers, and mixed
  complete/incomplete coefficient authority fail closed before streaming.

## Non-Changes

- No production Rust selector behavior changed in this package.
- No wepppy producer behavior changed in this package.
- Legacy parser support is retained.
- Legacy/non-Lane-D paths are retained for compatibility evidence.
