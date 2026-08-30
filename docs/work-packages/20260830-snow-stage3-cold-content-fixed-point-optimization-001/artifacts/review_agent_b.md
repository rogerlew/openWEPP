# Review B

Status: `COMPLETE — NO-GO (CORRECTION RE-REVIEWED)`

Evidence mode: `Static + Ran`

Reviewed diff: package baseline `792af753e7c936a66352ee69ef5c5c1a18447082`
through implementation commit `be40a943530c7ca25b8e899243e77300b92a7a56`,
plus the current package/prompt delegation amendments. The current worktree
contains concurrent package-review artifacts but no post-`be40a9435`
production, contract, or test edit.

## Executive assessment

The fixed-point implementation is bounded, matches the version-28 narrative,
and preserves authentic final replay. The new iterate reuses the existing
support-scaled/discrete-guarded contraction, falls back to the raw authentic
candidate when contraction is refused, retains authentic final LSE/boundary
operands and converged soil at the finalization restart, and suppresses only
one otherwise-converged relaxed Picard crossing. I found no production
diagnostic, receipt, restart, serialization, topology, tolerance, constitutive,
or public-output addition in the terminal executable diff.

The retained one-day log hash matches the artifact. Its reported widths sum to
491 accepted supports, cap signatures sum to 32, comparison-owner counts sum
to 45, discrete rejection count is zero, and the stated mass, energy, and
receipt residuals match the log and remain within unchanged thresholds. The
existing physical-outcome ledger reconstructs mass/ice/liquid/vapor/energy
from owner and receipt operands in a module that cannot feed solver operands;
its negative substitution tests provide anti-tautology protection rather than
relying only on the producer's acceptance bit.

Contract-first red evidence is credible: both retained logs show the final
contract-derived test symbols failing solely because their production helpers
were absent, and filesystem chronology places the contract/tests before the
helpers. Focused independent execution below also passes all three new vectors.

Package closure is nevertheless blocked by the findings below.

## Findings

### RB-001 — CRITICAL — required full/critical correctness regression is absent

The package declares this a critical kernel-numerics increment, and the diff
changes production kernel authority. The canonical strategy therefore requires
an immediate campaign-strength full correctness regression
(`testing-and-gate-strategy.md:240-264`; normally
`cargo nextest run --workspace --profile full`). Instead,
`gate-results.md:20-21` records a failed warnings-denied lint and a
failed/incomplete *orchestrator-only* run with 113 tests unexecuted. A source
`cargo check`, 19 focused tests, and 47 contract-consumer tests do not replace
the selected full/critical profile. The assertion at `gate-results.md:25-28`
that independent review is the sole blocker contradicts the recorded failures
and the gate-evidence non-deferral rule.

Disposition recommendation: `accepted`. Run the full workspace profile with
the repository-required stack environment, resolve or authoritatively
disposition every semantic failure, and replace the inaccurate sole-blocker
claim. This is closure-blocking.

### RB-002 — HIGH — warnings-denied increment lint failed

Increment closure requires warnings-denied lint for the affected package and
applicable reverse dependents (`testing-and-gate-strategy.md:137-155`). The
package records 773 errors at `gate-results.md:20` and labels them pre-existing,
but no prospectively declared exception, accepted baseline-delta lint policy,
or governing waiver exists. Under the package non-deferral rule, a required
`FAIL` cannot be converted into technical pass merely because the debt
predates this diff.

Disposition recommendation: `accepted`. Produce a passing canonical
warnings-denied result or obtain an explicit authority-level exception that
the repository permits; do not relabel the existing failure. This is
closure-blocking.

### RB-003 — HIGH — SC-SNOWENERGY-001 v28 does not complete the kernel-profile schema update

The v28 narrative and `INV-SNOWENERGY-054` are scientifically consistent with
the implementation (`SC-SNOWENERGY-001.md:1258-1285,1353-1366`), but the
contract revision does not add the new finalization-restart/stabilization
branches to the canonical `Branch and Guard Table`, does not map
`INV-SNOWENERGY-054` in the primary invariant/guard map, and does not add the
three new vector families to `Test Vector Obligations`. The kernel process
profile requires changed algorithm steps, branch table, guard/error mapping,
and test-vector obligations to be updated; its non-compliance rule forces
`HOLD`. `kernel-profile-compliance.md` currently declares `PASS` without
addressing these missing bindings.

Disposition recommendation: `accepted`. Amend the canonical tables/test-vector
section and the profile artifact without changing tolerances or runtime
semantics, then rerun affected contract admission/consumer checks. This is
closure-blocking contract governance, not a request for production changes.

### RB-004 — MEDIUM — canonical timing evidence is not bound to a terminal source identity

`implementation-test-evidence.md:20-53` records the baseline commit, command,
metrics, log path, and log hash, but not the terminal commit/tree or a dirty
worktree manifest. The retained log explicitly says the tree was dirty. The
production files predate the run and the resulting values match the committed
implementation, but that temporal inference does not satisfy the validation
strategy's minimum source-identity record for expensive result-bearing
evidence. Calling the run “exact-head” in `gate-results.md:25` is therefore
stronger than its retained provenance.

Disposition recommendation: `accepted`. Prefer a replacement canonical run
bound to a clean executable commit (governance-only dirt may be explicitly
excluded), or downgrade the claim and provide an accepted source-manifest
reconstruction if governance recognizes it as equivalent. This blocks the
exact-terminal performance/closure claim until resolved.

### RB-005 — MEDIUM — line-count governance is incomplete and its digest wording is inaccurate

`open_snow.rs` remains 2,726 lines. `crates/AGENTS.md:57-60` requires every
2,000+ line file to be recorded as `WARN` with decomposition rationale and
follow-up split intent. `line-count-governance.md:3,14-17` instead reports an
unqualified `PASS` and supplies neither. In addition, the stated digest
`6da236...` hashes baseline lines 342-870 including the separator blank line;
it is not the digest of the current 529-line include file. The semantic method
body is unchanged: baseline lines 342-869 and current include lines 1-528 both
hash to `97aec7cad748caac7a2b3c6fbf2c1023074495f6b4ce233c95893bb9bd10bdd5`.

Disposition recommendation: `accepted`. Record the required WARN,
decomposition rationale, owner/follow-up split intent, and distinguish the
transferred-block digest from the current semantic-body digest. No production
edit is needed.

## Independent commands run

- `env RUST_MIN_STACK=67108864 nix develop -c cargo test -p openwepp-hillslope-orchestrator finalization_restart_ --lib -- --nocapture`
  — `PASS`, 3/3.
- `bash tools/release/check_science_contract_admission.sh --base-ref 792af753e --worktree`
  — `A0_ADMITTED`, 49 contracts, four science surfaces, authority SHA-256
  `9987f528f6fe862635902f4b2df0b57857f6e79fe3554fef3d80cc2333da483f`.
- `git diff --check 792af753e..be40a9435` and current `git diff --check`
  — `PASS`.
- Static reconciliation of `/tmp/stage3_fp_cold/one-day-terminal-source.log`
  — SHA-256 and all reported counts, widths, rejection signatures, runtime,
  and closure values match the package artifact.

## Recommendation

`NO-GO / HOLD` for package completion. The implementation has no identified
physics, conservation, custody, phase, topology, receipt, rollback, diagnostic
persistence, or discrete-event correctness defect in this review, but
RB-001 through RB-004 are current-scope closure blockers. RB-005 also requires
artifact correction before final disposition. Re-review the amendments and
perform independent verification after all accepted findings are resolved.

## Correction re-review — `6953a36b8` plus source-order test binding

Evidence mode: `Static + retained Ran evidence`

Re-reviewed identity: clean correction commit
`6953a36b881e7167b47c76040208d1024818060a`, plus the sole current worktree
change in
`crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_tail_tests.rs`.
No heavy or canonical command was rerun by Review B. Retained log hashes were
recomputed and match the amended package artifacts.

### Finding dispositions

| Finding | Decision | Re-review status | Evidence and rationale |
|---|---|---|---|
| `RB-001` | `accepted` | `OPEN` | The required full profile was finally executed to completion: all 3,628 tests were attempted, with 3,503 passing, 96 failing, 29 timing out, and zero not-run (`nextest_full.log`, SHA-256 `dbdd682aa9c654f08955f65d7b74addfad999691be21c678ecd6da977f0b35ee`). This resolves the prior *absence/incompleteness* of execution but not the required correctness result. Three package-owned stale source-scan failures are corrected by the current binding and have a retained focused 5/5 pass. The other 122 failures/timeouts remain unproven as unrelated; downstream fixture location does not exclude causality from a kernel-numerics change. The full profile also predates the current test-only terminal diff. A passing exact-terminal full profile, or a governance-authorized disposition that actually satisfies the critical requirement, is still required. |
| `RB-002` | `accepted` | `OPEN` | The workspace warnings-denied run now reaches two early diagnostics (`openwepp-coupled-time` `filter_map_bool_then` and `openwepp-biogeochemistry` `similar_names`) before Cargo stops; log SHA-256 is `aac68d695f1d8f2e06f687c01aa199cc25d48f8d708a958763266e4323d11637`. This is a materially cleaner and more truthful result than the earlier 773-error artifact, but it remains `FAIL` and does not establish that these are the only workspace diagnostics. No passing warnings-denied result for the affected package/reverse dependents or governing exception exists. |
| `RB-003` | `accepted` | `CLOSED` | `SC-SNOWENERGY-001` v29 now binds the finalization restart, exactly-once stabilization, refusal, density, and authentic-publication behavior in the canonical branch table (`:745-747`), primary invariant/guard map (`:840`), formal vectors (`:1142-1153`), and child obligation map (`:1172`). The density reconciliation is internally consistent: an unpublished blend copies authentic-candidate density bitwise, reconstructs thickness, and still requires exact density equality before convergence. Existing density-only coverage plus the finalization density vector prevent tolerance/continuous-coordinate aliasing. The state-owning `CoveredFinalizationStabilizationV1` retains pending state across nonconvergence and consumes it once at the first otherwise-converged relaxed crossing. No floor, tolerance, cap, ledger, event, receipt, or publication rule changed. |
| `RB-004` | `accepted` | `CLOSED` | The replacement canonical run is bound to clean commit `6953a36b8` and passed with 491 accepted / 205 rejected, the identical width histogram, 32 caps, 45 scaled comparison rejections, zero discrete rejections, and bit-identical closure residuals. Body wall was 339.10 s. Log SHA-256 is `c6ba3bdb3a9bfd5d0bdd35e83fdb2f448dcd97dba67d70811d418e64cb856417`. The current worktree change is test-only and is excluded from the production dependency compiled by the one-day runner, so the clean executable evidence remains reusable. |
| `RB-005` | `accepted` | `CLOSED` | The line-count artifact now correctly reports `WARN`, the actual 2,721/529-line split, the matching semantic method-body digest `97aec7ca...`, decomposition rationale, named SnowEnergy owner, and next-authorized-touch extraction intent. This satisfies the 2,000-line governance rule while remaining below the mandatory 3,000-line threshold. |

### New finding RB-006 — MEDIUM — terminal package reconciliation has stale bindings

The current source-order correction is correct: after the mechanical split,
the five accepted-endpoint source-order tests must read
`open_snow_terminal_accepted_endpoint.rs`, and the one-line `include_str!`
change now binds the actual implementation. It is test-only, tightly scoped,
and its retained focused 5/5 pass addresses the three failures exposed by the
full profile.

The package's terminal evidence is not yet internally reconciled, however:

- `package.md` does not list `open_snow_tail_tests.rs` in the intended write
  set, while `owned-file-manifest.md` already claims it as reconciled;
- `gate-results.md` still summarizes the line split as 2,726/529 instead of
  the actual 2,721/529;
- `contract-implementation-evidence.md` still names the removed
  `covered_fixed_point_picard_accepts_convergence_v1` helper and associates
  consumer advancement with v28 before later describing v29; and
- that contract artifact retains the v28 authority digest `9987f528...`, while
  the v29 gate table records authority `a8828192...`.

Disposition recommendation: `accepted`. Add the source-order test path to the
authorized intended write set and correct the stale helper, version, line-count,
and authority-digest statements before terminal reconciliation and
verification. These are evidence/governance corrections; no production or
science change is indicated.

### Re-review recommendation

`NO-GO / HOLD` remains the only truthful disposition. `RB-003`, `RB-004`, and
`RB-005` are closed, and the source-order binding is correct. `RB-001` and
`RB-002` remain closure-blocking failed requirements, not merely missing runs.
`RB-006` must also be reconciled before verification. The clean canonical
one-day evidence continues to support the optimization and unchanged closure,
but it cannot substitute for a passing critical full profile or
warnings-denied gate.

## Final bounded re-review — RB-006 generation-37 correction

Evidence mode: `Static + retained Ran evidence`

Scope was limited to the four stale reconciliation bindings identified in
`RB-006`; no heavy or canonical gate was rerun by Review B.

`RB-006` disposition: `accepted — CLOSED`.

- `package.md` and `owned-file-manifest.md` both authorize and inventory
  `open_snow_tail_tests.rs`.
- `gate-results.md` records the actual 2,721/529-line split.
- `contract-implementation-evidence.md` names the stateful
  `CoveredFinalizationStabilizationV1` seam and consistently identifies
  SnowEnergy authority/consumers as version 29.
- The generation-37 impact map binds the mechanically split accepted-endpoint
  test surface by exact path, and `terminal-diff-reconciliation.md` identifies
  generation 37 and that surface.
- Contract evidence and the gate table agree on the exact-source A0 result:
  `A0_ADMITTED`, 49 contracts, four science surfaces, authority SHA-256
  `ce2befbdb7214be8194f01d3f8645663ce916a232ff476cc21692986034dad1a`.

The write-set, implementation binding, line-count, authority-policy, and A0
records now reconcile; no residual `RB-006` defect remains. `RB-001` and
`RB-002` remain open and closure-blocking, so Review B's final recommendation
remains `NO-GO / HOLD`.
