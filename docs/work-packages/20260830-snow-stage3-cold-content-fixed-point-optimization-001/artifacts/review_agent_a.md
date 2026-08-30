# Review A

Status: `CORRECTION RE-REVIEW COMPLETE — NO-GO`

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

---

## Correction re-review — commit `6953a36b8` plus source-order test binding

Evidence mode: `Static + retained Ran evidence`

Reviewed correction identity:
`6953a36b881e7167b47c76040208d1024818060a`, plus the sole current worktree
change in
`crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_tail_tests.rs`.
That one-file diff had SHA-256
`da1bca7bf268565db7cba44d1e31c41fb6bdde9b06c5cff639d397a6fdd023ec`
at review time. No heavy or canonical gate was rerun by Review A.

### Finding dispositions

| Finding | Decision | Re-review result | Evidence |
|---|---|---|---|
| `RA-001` | accepted | `CLOSED` | `SC-SNOWENERGY-001` v29 now states the version-23 rule unambiguously: density is copied bitwise from the authentic candidate and never interpolated, while a density difference remains exactly nonconverged until an authentic image matches. The branch table, `INV-SNOWENERGY-054`, guard map, primary binding, and test-vector obligations agree with `covered_fixed_point_stage3_underrelaxed_iterate_v1`. The finalization vector at `open_snow_convergence_tests.rs:180` proves candidate-density bit identity and continued nonconvergence. |
| `RA-002` | accepted | `STILL OPEN` | The superseding full profile is execution-complete—all 3,628 tests were attempted—but its outcome is `FAIL`: 3,503 pass and 125 fail. The warnings-denied workspace Clippy run also remains `FAIL`, stopping on `filter_map_bool_then` in `openwepp-coupled-time` and `similar_names` in `openwepp-biogeochemistry`. Complete execution resolves the earlier `NOT RUN` portion of RA-002; it does not convert either mandatory gate into a pass. |
| `RA-003` | accepted | `CLOSED` | The formerly free boolean predicate is now the state-owning `CoveredFinalizationStabilizationV1` used directly by production at `open_snow.rs:1728`, `:1922`, and `:2110`. Its test exercises restart observation, persistence through nonconvergence, exactly-once consumption, the following accepted crossing, and raw exact-floor nonactivation (`open_snow_convergence_tests.rs:237`). Static source confirms that only an actually changed relaxed finalization iterate arms the seam; retained clean-commit canonical evidence confirms authentic finalization/replay and unchanged closure. |

### Source-order test-binding assessment

The current one-file test correction is technically correct and behavior
preserving. After the mechanical split, the accepted-endpoint implementation
is no longer textually present in `open_snow.rs`; it lives in
`open_snow_terminal_accepted_endpoint.rs` through `include!`. Pointing
`accepted_branch_source()` directly at that authoritative include restores the
ordering, poison, rollback, and no-physics-rerun assertions to the production
method they intend to inspect. The retained focused result is 5/5 pass, run ID
`d72d27a7-7634-48cd-b0e1-d314de34e06e`. I found no new arithmetic,
publication-order, or rollback-semantics issue in this test-only diff.

The terminal owned-file manifest and package intended write set now both
include `open_snow_tail_tests.rs`. Impact-map generation 37 also adds one exact
critical SnowEnergy binding for that path. The current full-profile result
predates this test-only diff; the focused 5/5 result closes the three
package-owned stale source-scan failures, but it does not erase the other
recorded full-profile failures.

### Heavy-evidence reconciliation and residual risk

- Full correctness profile: `FAIL`, all 3,628 attempted, 3,503 passed, 125
  failed, zero not-run; elapsed `5,022.73 s`; retained log SHA-256
  `dbdd682aa9c654f08955f65d7b74addfad999691be21c678ecd6da977f0b35ee`.
- Warnings-denied workspace Clippy: `FAIL`, exit 101 after two early lint
  diagnostics; retained log SHA-256
  `aac68d695f1d8f2e06f687c01aa199cc25d48f8d708a958763266e4323d11637`.
- Three full-profile failures were the stale package-owned source scan and are
  now covered by the corrected 5/5 focused run. The other 122 failures/timeouts
  and both lint diagnostics remain outside this package's current correction
  write set, but the gate non-deferral rule does not permit Review A to relabel
  them as passing or waived.
- The clean-commit canonical rerun at `6953a36b8` preserves 491 accepted / 205
  rejected trials, 32 caps, 49 exact-floor supports, zero discrete comparison
  rejections, and unchanged mass/energy/receipt closure. RA-001/RA-003
  corrections therefore show no canonical numerical regression.

### Current recommendation

`NO-GO` / `HOLD`. `RA-001` and `RA-003` are closed, and the source-order
test-binding correction is accepted with no technical finding. `RA-002`
remains closure-blocking because both mandatory heavy gates completed with
failed outcomes. Dual verification and final `COMPLETE` disposition must wait
for an authority-compliant resolution of those failures and exact terminal
write-set reconciliation.

---

## Final bounded package-local closure check

Evidence mode: `Static`

Checked the current worktree after the tail-test and impact-map generation-37
correction. No heavy or canonical command was rerun.

- `package.md` and `owned-file-manifest.md` both authorize and record
  `open_snow_tail_tests.rs`.
- The test reads the authoritative mechanically split
  `open_snow_terminal_accepted_endpoint.rs`; retained focused evidence is 5/5
  pass.
- `tools/release/authority-policy/impact-map.json` is valid JSON at generation
  37 and has exactly one critical SnowEnergy exact-path entry for
  `open_snow_tail_tests.rs`.
- Line counts, the stateful helper name, contract version 29, authority digest,
  and the clean canonical source identity are reconciled in their owning
  artifacts.
- `git diff --check` passes on the current worktree.

The final terminal-diff reconciliation now names authority impact-map
generation 37 and explicitly includes the mechanically split accepted-endpoint
test surface. The previously noted one-line evidence residue is therefore
closed; no package-local reconciliation defect remains.

Current finding state:

- `RA-001`, `RA-003`, `RB-003`, `RB-004`, and `RB-005`: `CLOSED`.
- `RB-006`: `CLOSED`; intended/owned paths, line counts, helper/version/digest
  statements, impact-map binding, and terminal generation wording reconcile.
- `RA-002`, `RB-001`, and `RB-002`: `OPEN` and closure-blocking because the
  complete full profile and workspace warnings-denied Clippy outcomes are
  failures.

All package-local review findings are closed. Final bounded verdict nonetheless
remains `NO-GO / HOLD`: the workspace full-profile and warnings-denied Clippy
failures (`RA-002`, `RB-001`, and `RB-002`) still prevent verification PASS and
package `COMPLETE`.
