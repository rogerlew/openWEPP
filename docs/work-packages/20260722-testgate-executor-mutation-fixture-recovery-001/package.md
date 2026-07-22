# TESTGATE Executor Mutation Fixture Recovery

Package ID: `20260722-testgate-executor-mutation-fixture-recovery-001`

Queue ID: `TESTGATE-EXECUTOR-MUTATION-FIXTURE-RECOVERY-01`

Status: `ACTIVE / SCAFFOLD`

## Objective

Close RTR-037 by aligning the root-only executor mutation fixture with global
adjudicated CRAP while preserving every source-mutation and prerequisite-blocking
assertion.

## Declared Write Set

- `crates/openwepp-gate-planner/src/executor.rs`
- `docs/work-packages/20260722-testgate-executor-mutation-fixture-recovery-001/**`
- `docs/work-packages/20260722-testgate-affected-crap-planner-escalation-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before the fixture edit.
2. Replace only the invalid affected-CRAP definition with global adjudicated
   CRAP; retain mutation, invalid receipt, and blocked-node assertions.
3. Run the exact failed test, complete planner target at the changed head,
   formatting, Clippy, package audit, and dual review.
4. Close RTR-037 and resume RTR-035 closure.

## Exit Criteria

- The root fixture does not claim affected quality.
- Existing mutation and receipt assertions are unchanged and passing.
- Exact/owning tests, Clippy, package audit, and dual review pass.
- RTR-037 closes durably before qualification.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers.
Expected outputs are package-local reviews; write access is read-only. Do not
push, deploy, switch branches, manually dispatch TESTGATE, run HEAVY, or repeat
unchanged expensive gates.
