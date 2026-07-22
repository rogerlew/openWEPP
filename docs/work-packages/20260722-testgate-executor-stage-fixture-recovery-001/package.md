# TESTGATE Executor Stage Fixture Recovery

Package ID: `20260722-testgate-executor-stage-fixture-recovery-001`

Queue ID: `TESTGATE-EXECUTOR-STAGE-FIXTURE-RECOVERY-01`

Status: `ACTIVE / SCAFFOLD`

## Progress

- [x] Scaffold commit `a3c80219` predates the fixture edit.
- [x] Replace only the invalid affected-CRAP fixture identity.
- [x] Exact regression, formatting, and target Clippy pass.
- [ ] Commit, run the complete owning target, package audit, and dual review.
- [ ] Close RTR-036 and resume RTR-035 closure.

## Objective

Close RTR-036 by aligning one executor-stage selection fixture with the canonical
global-quality contract for its root-only measurement package. Preserve every
LIGHT, final-receipt, unknown-stage, and HEAVY-audit assertion.

## Declared Write Set

- `crates/openwepp-gate-planner/src/executor_coverage_tests.rs`
- `docs/work-packages/20260722-testgate-executor-stage-fixture-recovery-001/**`
- `docs/work-packages/20260722-testgate-affected-crap-planner-escalation-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before the fixture edit.
2. Replace only the affected-CRAP fixture definition with global adjudicated
   CRAP; retain all stage-selection assertions and ordering.
3. Run the exact failed test, the complete planner target from a clean commit,
   formatting, Clippy, package audit, and dual review.
4. Close RTR-036 at the exact correction commit, then resume RTR-035 review.

## Exit Criteria

- The fixture no longer constructs affected CRAP for root `openwepp`.
- All existing assertions remain unchanged.
- Exact and owning tests, Clippy, package audit, and dual review pass.
- RTR-036 is durably closed before qualification.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers.
Expected outputs are package-local reviews; write access is read-only. Do not
push, deploy, switch branches, manually dispatch TESTGATE, run HEAVY, or repeat
unchanged expensive gates.
