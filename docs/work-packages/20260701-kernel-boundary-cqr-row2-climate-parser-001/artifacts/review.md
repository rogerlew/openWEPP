# Review

Evidence mode: Static + Ran.

## Reviewer A

Findings: none blocking.

- The row #2 change is test-only CQR work. It restores assertions at the stable
  climate parser/runtime-input boundary without changing parser grammar, typed
  projection code, formulas, or public output schemas.
- Full-workspace CRAP-after reports `0` row #2 owned production functions above
  CRAP 30. No ADR-0021 warning disposition is needed.
- H2637 identity and `compatibility_edge_invocations=0` prove the row did not
  reintroduce compatibility runtime edges or mutate protected outputs.

Residual risk:

- The row leaves unrelated workspace CRAP offenders outside the row #2 write
  set. Full-workspace above-threshold count is `266`, but row #2 scope is clean.

## Reviewer B

Findings: none blocking.

- The restored tests cover the secondary execplan requirement directly through
  public `HillslopeClimateRuntimeRequest` accessors: non-breakpoint forcing,
  breakpoint forcing, datver-0 override behavior, itemp runtime rejection, and
  direct-day out-of-range errors.
- Assertions verify precipitation reconstruction closure from the typed
  intensity/time vectors instead of pinning private symbol-surface internals.
- Production `03_climate.rs` was not edited; the watched `direct_day_forcing`
  function improved from 60% to 100% coverage and stayed well below the
  ADR-0021 threshold.

Residual risk:

- The row does not claim climate parser grammar expansion or new physics
  behavior. It only restores focused secondary coverage while preserving current
  behavior.

## Disposition

Status: `PASS`.

Row #2 may close as `EXECUTED-COMPLETE-ROW2-CQR`.
