# Review A

Status: `COMPLETE — NO-GO`

Evidence mode: `Static + Ran`

Reviewed implementation identity: `792af753e..be40a9435` (`be40a9435` is the
committed terminal implementation). Also reviewed the dirty-worktree
review-closure governance amendment in `package.md` and `prompts/`; its tracked
diff SHA-256 at review time was
`e5110c65cd966753ef33fcb863bb738ac43b4edbf831e1156d583aa094808be2`, plus the
new active kickoff prompt. No production, contract, or test file had a
worktree change during this review.

## Findings

### RA-001 — HIGH — The finalization relaxation contradicts the canonical exact-density guard

`SC-SNOWENERGY-001` says relaxation is prohibited across any density change
and `INV-SNOWENERGY-054` permits the new finalization update only across exact
density identities
(`docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md:1277`
and `:1353`). The reused implementation has no left/right density-equality
guard. Instead it deliberately copies the authentic candidate density while
blending mass, liquid, cold content, and refreeze across a density change
(`crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs:344`
and `:381`). The pre-existing focused test explicitly requires a one-ULP
density change not to disable damping
(`crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs:66`).
The new helper at `fixed_point.rs:488` extends that behavior to finalization,
so this is in the reviewed semantic increment even though the underlying
under-relaxation helper predates it.

Impact: production behavior and the promoted v28 contract disagree at an
exact structural/discrete guard. That prevents science-contract promotion and
makes it ambiguous whether a density transition may be crossed by a mixed
mass/energy iterate.

Disposition recommendation: `accepted`. Resolve contract-first. Either enforce
bitwise left/right density equality and replace the contrary test, or amend the
canonical authority to state explicitly that density may advance atomically
from the authentic candidate while only continuous coordinates are blended.
The latter appears consistent with the version-23 change-log intent, but that
intent does not override the current unambiguous prohibition. Add a
finalization-specific density vector and rerun focused plus canonical evidence
after the authority/code/test reconciliation.

### RA-002 — HIGH — Required critical-regression and warnings-denied evidence is failed/incomplete

This increment semantically changes production kernel numerical authority, so
the critical rule requires an immediate campaign-strength full correctness
regression (`docs/standards/testing-and-gate-strategy.md:240`). Increment
closure also requires warnings-denied lint for the affected package and
reverse dependents (`testing-and-gate-strategy.md:137`). The retained gate
table instead records warnings-denied Clippy as `FAIL` with 773 errors and the
broad orchestrator nextest run as `FAIL/INCOMPLETE`, with 113 tests not run
(`artifacts/gate-results.md:20`). The broad run also omitted the repository's
required enlarged stack environment. Nevertheless, the artifact says that
independent evidence is the sole completion blocker (`gate-results.md:25`),
which conflicts with the non-deferral rule that any required `FAIL` or
unjustified `NOT RUN` prevents completion
(`docs/work-packages/AGENTS.md:63`).

Impact: the focused 19-test suite and one-day fixture provide strong local
evidence, but they cannot establish absence of cross-domain regressions for a
critical production-kernel change. The current evidence also cannot support a
claim that review/verification is the only remaining blocker.

Disposition recommendation: `accepted`. Run the selected campaign-strength
correctness regression on the exact terminal source with
`RUST_MIN_STACK=67108864` and carry every deterministic failure to correction
or an explicitly authorized authority-backed disposition. Reconcile the
warnings-denied requirement rather than labeling the existing failure
nonblocking solely because the backlog predates this diff. Until then, retain
`HOLD` and correct the gate/disposition narrative.

### RA-003 — MEDIUM — The new stabilization state machine lacks a behavioral seam test

The production loop sets `finalization_stabilization_pending` only after a
damped finalization restart, retains it through nonconverged Picard iterations,
consumes it at the first otherwise-converged crossing, performs the extra
support-scaled update, and then permits another authentic finalization
(`crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs:1922`,
`:1993`, and `:2108`). The new test named
`finalization_restart_requires_one_relaxed_picard_stabilization_crossing`
tests only four truth-table outputs of the pure acceptance helper
(`open_snow_convergence_tests.rs:213`). It cannot fail if the loop stops setting
the pending flag, clears it too early, applies two extra updates, or routes a
relaxed intermediate toward publication. The aggregate one-day count detects
large performance drift but does not isolate this control invariant.

Impact: the central fix can regress while all contract-derived unit tests still
pass, including the claimed “exactly one” behavior and authentic-map-only
publication condition.

Disposition recommendation: `accepted`. Add a bounded loop/seam test with a
forced authentic finalization mismatch that observes one—and only one—extra
guarded Picard update before retry, verifies exact-floor raw behavior, and
proves the installed endpoint comes from authentic finalization/replay rather
than the relaxed intermediate.

## Residual risk and missing tests

- The retained exact-head one-day evidence shows material improvement
  (491/205 trials, 32 caps), zero discrete comparison rejections, unchanged
  exact-floor use, and mass/energy/receipt closure within unchanged bounds.
  It does not resolve RA-001 or replace the missing critical regression in
  RA-002.
- The 60-second constant, 96-iteration bounded termination, typed cap failure,
  raw exact-floor behavior before authentic period-two detection, and exact
  receipt/publication replay remain present in the reviewed source.
- The finalization helper falls back to the raw authentic candidate when the
  existing relaxation helper declines reconstruction; no new clamp, tolerance,
  serialization field, persisted diagnostic, or public output was introduced.
- The terminal accepted-endpoint file split is mechanically equivalent to the
  baseline method body apart from the necessary enclosing `impl` close; no
  duplicated production implementation was introduced by that split.

Ran independently:

`env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(covered_convergence_policy_tests)'`

Result: `PASS`, 19/19, nextest run ID
`7d44c9de-ea1a-4077-9873-5130722b0a2a`.

## Recommendation

`NO-GO` / `HOLD`. Findings RA-001 and RA-002 are closure-blocking. RA-003
should be fixed in the same package because it directly protects the newly
contracted solver transition. No approval is issued until accepted findings
are corrected, dispositioned, and independently verified.
