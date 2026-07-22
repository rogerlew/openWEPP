# TESTGATE Affected CRAP Authority Re-expression

Package ID: `20260722-testgate-affected-crap-authority-reexpression-001`

Queue ID: `TESTGATE-AFFECTED-CRAP-AUTHORITY-REEXPRESSION-01`

Status: `ACTIVE / SCAFFOLD`

## Progress

- [x] Scaffold commit `ae5f33b1` predates implementation edits.
- [x] Seal checker scope bytes at one read and final publication boundary.
- [x] Align reconstructed Cargo production targets with checker admission.
- [x] Re-express global fixtures and measurement-only assertions under helpers.
- [x] Focused classification/planner/fixture regressions pass 9/9 total.
- [x] Core aggregate correction commit `04f3b619`; package audit `READY`.
- [x] Correct RTR-041; exact changed-head reconstruction passes in 476.228s.
- [ ] Run changed-head owning target/Clippy and dual review.
- [ ] Close RTR-035 through RTR-040 and delegate qualification.

## Objective

Close RTR-039 and RTR-040, and provide one prospective exact authority for the
final RTR-035 through RTR-038 correction, by re-expressing root/global fixture
selection and measurement-only assertions under shared helpers while aligning
the planner's production classifier with affected CRAP preflight.

## Correction Authority Envelope

- RTR-039: split package authority at commit `92f35b24`.
- RTR-040: planner/checker production-classifier drift.
- Allowed changes: reconstructed Cargo target classification, planner selection
  tests, test-only global-quality assertions, and test fixture helper extraction.
- Protected behavior: no science, gate thresholds, executor stages, receipts,
  retry, coverage acquisition, comparator, or deployment behavior may change.

## Declared Write Set

- `crates/openwepp-gate-planner/src/repository.rs`
- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/executor_coverage_tests.rs`
- `docs/work-packages/20260722-testgate-affected-crap-authority-reexpression-001/**`
- `docs/work-packages/20260722-testgate-affected-crap-root-scope-recovery-001/**`
- `docs/work-packages/20260722-testgate-affected-crap-planner-escalation-001/**`
- `docs/work-packages/20260722-testgate-executor-stage-fixture-recovery-001/**`
- `docs/work-packages/20260722-testgate-executor-mutation-fixture-recovery-001/**`
- `docs/work-packages/20260722-testgate-executor-receipt-fixture-recovery-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before implementation edits.
2. Derive planner production ownership from reconstructed Cargo target metadata:
   direct `crates/*` manifest, library/binary/proc-macro kind, and owned `src/`
   target path.
3. Add production positive and test-only/out-of-tree negative classification
   coverage; retain root-global and ordinary-production affected planning.
4. Re-express executor global-quality fixtures and planner measurement-only risk
   assertions through shared helpers without changing assertions.
5. Run focused tests, exact planner target, formatting, Clippy, package audit,
   line-count governance, and dual independent review.
6. Close RTR-035 through RTR-040 only at the exact reviewed correction commit,
   then rebuild and delegate exactly one changed-head qualification.

## Exit Criteria

- Planner and checker production admission agree for positive, root, test-only,
  nested, and out-of-tree target shapes.
- One package audit is `READY` for the complete exact correction diff.
- All root-only fixtures use the shared global-quality fixture helper and retain
  their prior assertions.
- Focused tests, planner target, Clippy, formatting, line-count governance, and
  dual review pass.
- RTR-035 through RTR-040 close durably before qualification.
- RTR-041 closes only after the exact reconstruction seam and owning target pass.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, one
comparator runner for exact qualification, and two independent read-only
terminal verifiers. Expected outputs are package-local reviews/verifications and
retained external qualification evidence. Write access is read-only except for
the comparator's ignored evidence root. Do not push, deploy, switch branches,
manually dispatch TESTGATE, run HEAVY on the parent, or repeat unchanged gates.
