# PERFIDX03 Disposition

Status: HOLD 2026-06-17
Evidence mode: **Ran** + **Static**

## Result

PERFIDX03 is held, not complete.

The diverse registry precondition passed after fixing frost fine-layer coverage
and irrigation sidecar coverage. The indexed authority flip was attempted and
showed no logical output divergence on the exercised cases, but the active path
regressed OFE5 wall-clock from a baseline mean of `27.01s` to an active-flip mean
of `38.34s`.

The regression is explained by the current compatibility seam: the sparse store
is cloned, then exported back into full `BTreeMap` state/flux maps for the kernel
execution path. That export cost prevents Stage 3 from realizing the clone win.

Before disposition, production authority activation was disabled. The current
tree retains registry coverage fixes and inactive indexed-authority support, and
the no-flip OFE5 sample returned to baseline range at `26.80s`.

## Gate Summary

- Pre-flip diverse registry coverage: PASS.
- Authority flip: FAIL/HOLD.
- Full H2637 both-UI + OFE1-OFE5 bit-identity anchor: NOT RUN after speed failed.
- Exercised-case logical identity: PASS/PARTIAL.
- Realized speedup: FAIL.
- Required Rust gates: PASS.
- Line-count governance: WARN, no required refactor.

## Closure Decision

Do not merge/ship PERFIDX03 as a completed authority flip. The next package must
close the indexed-kernel-seam/export-cost blocker, then re-run the full PERFIDX03
anchor and speed gates.

## Post-review closure (2026-06-17, operator-approved)

The sections above describe the tree **as executed**. After review the decision was
to **discard all PERFIDX03 working-tree code** rather than land the held flip
plumbing — the flip regressed and `PERFIDX03B` reworks the seam from scratch anyway,
so there was nothing in the code worth carrying forward. The committed PERFIDX03
record is therefore **docs-only** (this disposition + the evidence/handoff artifacts
+ the independent review); `crates/` returns to the PERFIDX02-complete state.

Two findings were extracted from the discarded code:
1. **The flip blocker** → `PERFIDX03B` (remove the per-lane/day full-`BTreeMap`
   export at the kernel seam; have the seam consume the indexed rep / a cached
   export). The seam read-migration is coupled to the flip — part of Stage 4 must
   land *with* the re-flip.
2. **Irrigation wiring** (inadvertently activated by the registry-coverage gate) →
   `docs/backlog/20260617-irrigation-management-gated-activation.md`. Irrigation is
   deferred future work, runs **only when the management declares it**, and is out of
   scope for the perf migration. The registry-completeness gate must **not** wire or
   activate irrigation to satisfy coverage.
