# TESTGATE Verifier READY-Audit Fixture Closure

Package: `20260722-testgate-verifier-ready-audit-fixture-001`
Status: `IMPLEMENTED-REVIEW-PENDING`
Defect: `RTR-029`
Campaign: `TESTGATE-RECOVERY-TRUST-01`

## Objective

Close `GATE-VERIFIER-READY-AUDIT-CURRENT-DIFF-COUPLING` by making the verifier
READY-audit characterization use a self-owned package-authority fixture rather
than assuming the ambient repository's `HEAD^..HEAD` contains exactly one
changed `package.md`.

## Correction Authority Envelope

- Observed violation: the rank-7 planner baseline failed at
  `ready_audit_verification_preserves_order_and_exact_verdict` with zero package
  authorities after a prompt-only commit.
- In-scope modules: test-only fixture code in
  `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs` and crate-scoped
  `#[cfg(test)]` visibility for the existing isolated executor fixture in
  `crates/openwepp-gate-planner/src/executor.rs`.
- Allowed edit: reuse the isolated committed repository/package fixture and
  preserve the existing READY-audit success and error-order assertions. The
  executor amendment is visibility-only inside `#[cfg(test)]`; no production
  item or compiled production byte may change.
- Acceptance: the focused verifier READY-audit tests pass from a head whose
  latest commit does not change a package file; the fixture leaves no repository
  or process state behind.
- Protected boundaries: no production verifier, planner, executor, schema,
  policy, package-validation, receipt, or audit semantics may change. Copying
  the full fixture into a second module is rejected because it would create two
  trust-fixture implementations that can drift.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/defect_closure_execplans.md`
- `docs/standards/testing-and-gate-strategy.md`
- `crates/openwepp-gate-planner/src/verifier.rs`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/executor.rs` (`#[cfg(test)]` fixture
  visibility only)

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent implementation reviewers and two
terminal verifiers. Outputs are package-local review and verification
artifacts. Write access is read-only.

## Declared Write Set

- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
- `docs/work-packages/20260722-testgate-verifier-ready-audit-fixture-001/**`
- `docs/work-packages/20260721-cqr-testgate-recovery-07-planner-001/**`
- `docs/work-packages/20260721-cqr-testgate-recovery-closeout-execplan.md`
- `/home/workdir/testgate-history/recovery-trust-01-attempts.jsonl`

## Phase Plan

1. Retain the failed baseline and durably open RTR-029.
2. Replace ambient-head package admission with a self-owned isolated fixture.
3. Run formatting and focused READY-audit characterization.
4. Obtain dual independent implementation review and correct findings.
5. Commit the exact correction, durably close RTR-029, and complete dual
   terminal verification.
6. Return rank 7 to execution without an unchanged metric retry.

## Exit Criteria

- The focused READY-audit success and rejection paths pass on the exact
  correction head.
- The test no longer requires any package path in the ambient `HEAD^..HEAD`.
- No process, temporary directory, worktree, or repository mutation leaks.
- Dual review and dual terminal verification pass.
- RTR-029 is durably closed only after the exact correction commit exists.

## Current Evidence

- Ran: focused READY-audit characterization passed 1/1 in 207.468 seconds.
- Ran: `cargo fmt --all -- --check` passed.
- Static: executor production prefix SHA-256 is unchanged at
  `eb481c992b73419ce76fe8beff7e437c9a06b805db3e47d2673ef0bf68386098`.
- Ran: no fixture repository, artifact directory, ledger, or child process was
  retained after the focused test.
- Static: review A found that the earlier amendment described the executor test
  visibility edit in the authority envelope but omitted its exact path from the
  binding Declared Write Set. RTR-030 owns that annotation defect; this exact
  bullet is its canonical correction.

## Security Impact Gate

- security_impact: high
- dedicated_security_review_required: no
- rationale: test-only trust-path characterization receives dual independent
  review and production semantics are protected from change.
