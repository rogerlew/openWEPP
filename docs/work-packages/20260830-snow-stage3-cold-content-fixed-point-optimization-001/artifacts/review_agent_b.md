# Review B

Status: `COMPLETE — NO-GO`

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
