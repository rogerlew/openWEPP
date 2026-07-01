# Review

Evidence mode: Static + Ran.

## Reviewer A

Findings: none blocking.

- The row #9 changes are behavior-preserving CQR work. They add test access to
  existing direct-runtime helpers, split guard groups, and factor trace writer
  and surface ET demand helper logic without changing public schema, executor
  selection, or process-physics formulas.
- Full-workspace CRAP-after reports `0` row #9 owned production functions above
  CRAP 30. No ADR-0021 warning disposition is needed.
- H2637 identity and `compatibility_edge_invocations=0` prove the row did not
  reintroduce compatibility runtime edges or mutate protected outputs.

Residual risk:

- The row leaves unrelated workspace CRAP offenders outside the row #9 write
  set. Full-workspace above-threshold count is `298`, but row #9 scope is clean.

## Reviewer B

Findings: none blocking.

- The added `cqr_row9_direct_runtime_tests` cover the stable typed surfaces that
  were missing for this row: PMET compute/storage terms, staged evaporation,
  R4N surface ET PMET/manual branch selection, direct constructor validation,
  snow/frost carries, day commit layer-source priority, and R4A frost storage
  rebalance.
- The day constructor validator split preserves the same error fields and keeps
  the orchestration fail-closed posture.
- R7H trace writer decompositions retain the same environment-gated append
  behavior while reducing the parent writer CRAP ceiling.

Residual risk:

- The row does not claim process-physics parity improvements. It only closes CQR
  debt and asserts existing direct-runtime behavior.

## Disposition

Status: `PASS`.

Row #9 may close as `EXECUTED-COMPLETE-ROW9-CQR`.
