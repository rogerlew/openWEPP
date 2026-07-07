# Codex Review - LANED-T3 Rev-31 Ratification/Solve-Cost

Evidence:
- Static: reviewed current diff and changed Rust/contract/package artifacts.
- Ran: `git diff --check` (pass).
- Not run: cargo tests, clippy, full nextest, deny, markdown/contract lint.

## Findings

### Medium - Parent pointers still describe the removed Filippov commit path

Files:
- `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/worker-handoff.md:46`
- `docs/work-packages/README.md:57`

The modified parent-package pointers still tell future workers that the key
T3 discovery is `LOW->HIGH->Filippov closure` / `basin-split/Filippov closure
rules`. That contradicts the current canonical rev-29/rev-31 posture in
`SC-OFEROUTE-001`: the filled-jump commit was removed, LOW jump must recover a
real HIGH-branch root, and both-branches jump/double collapse now fails closed.

This is not a runtime defect in the rev-31 Rust diff, but it is contract-facing
handoff drift on branch identity/fail-closed semantics. Amend these pointers to
say LOW-jump -> HIGH-root recovery and fail-closed double-collapse, not
Filippov closure.

## Question Responses

1. Branch-local warm seeding: I do not see a path where it changes branch
identity, convergence target, or fail-closed behavior relative to rev-29. The
seed is derived only from the same implicit downstream march's already-solved
upstream discharge (`implicit_recession.rs:152`), is filtered by branch side
before use (`implicit_recession.rs:234` and `:256`), and the LOW-first then
HIGH fallback with double-collapse fail-closed remains intact
(`implicit_recession.rs:296` and `:317`).

2. New profile counters: output-safe. The counters live in the opt-in
thread-local routing profile snapshot (`profile.rs:52`), are no-op unless
profiling is enabled (`profile.rs:129`), count fixed-point map and branch
residual evaluations only (`kinematic_wave.rs:288`,
`implicit_recession.rs:333`), and are emitted only in the active profile
stderr line (`05_runner_execution_and_outputs.rs:168`). They do not alter
published outputs, manifests, or solver control flow.

3. Hybrid Case-4 D-val harness: valid as a retained ratification vector for
the intended single-OFE hybrid composition. It runs the active source phase
explicitly and the post-cutoff drain phase implicitly from the same state
(`dval.rs:319` and `:338`), then checks the same Iwagaki oracle tolerances in
the retained test (`d10b_reconciliation_tests.rs:94`). Marking it ignored is
legitimate because the hold evidence is explicit: the package records the
failed ladder errors and states that no selector promotion is authorized
(`artifacts/ratification-evidence.md:30`).

4. Rust correctness: no blocking issue found in mass bookkeeping, closure
captures, lifetimes, tests, or the changed public surfaces. The implicit step
continues to book inflow/source/outflow/storage through the same ledger fields
(`implicit_recession.rs:128`, `:162`, `:171`, `:194`), and the hybrid harness
uses local closures whose borrowed config/mesh lifetime stays within the run.

## Residual Risk And Missing Tests

- The package is correctly held on Case-4 hybrid fidelity failure:
  `22.8% / 15.5% / 10.2%` peak errors versus the `5%` tolerance.
- Closure gates remain incomplete in the package artifact: markdown/doc lint,
  contract/profile checks, protected-output off-surface audit, clippy, full
  nextest, deny, and line-count governance are not recorded as passed.
- I did not independently rerun the ignored Case-4 vector or the H2637 timing
  command in this review.

## Verdict

GO-WITH-AMENDMENTS.

The rev-31 Rust changes are acceptable for an experimental/unpromoted held
package, but amend the stale Filippov wording before treating the package
handoff/catalog as contract-aligned.
