# TESTGATE Clippy Test-Length Recovery

Package ID: `20260722-testgate-clippy-test-length-recovery-001`

Queue ID: `TESTGATE-CLIPPY-TEST-LENGTH-RECOVERY-01`

Status: `ACTIVE / READY-QUALIFICATION`

## Progress

- [x] Scaffold commit `3e9a1427` predates both Rust test edits.
- [x] Mechanically split the planner test from 157 to 42 lines.
- [x] Mechanically split the verifier test from 101 to 75 lines.
- [x] Formatting, both exact focused tests, package Clippy, and exact workspace
  Clippy pass.
- [x] Complete dual independent review at exact correction commit `8b26689c`.
- [x] Close RTR-033 at exact correction commit `8b26689c`; durable ledger entry
  `1a40c57e`.
- [ ] Delegate one changed-head qualification and dual terminal verification.

## Objective

Close RTR-033 by mechanically extracting cohesive setup/assertion helpers from
the two CQR characterization tests rejected by workspace Clippy's 100-line
test-function limit. Preserve every assertion, fixture input, ordering, and
production byte.

## Observed Failure

Receipt `20038867...fd4` recorded workspace Clippy exit 101:

- `planner_coverage_tests.rs`: graph-selection test, 157 lines;
- `verifier_coverage_tests.rs`: retry/audit/binding guard test, 101 lines.

The attempt ran each selected node once. Downstream doctest, full Nextest, and
CRAP were blocked and did not run.

## Declared Write Set

- `crates/openwepp-gate-planner/src/planner_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
- `docs/work-packages/20260722-testgate-clippy-test-length-recovery-001/**`
- `docs/work-packages/20260722-cqr-aggregate-admission-validator-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Commit this scaffold before editing either Rust test module.
2. Extract behavior-preserving helpers until both test functions are below 100
   lines; do not change production code or test assertions.
3. Run formatting, the exact two focused tests, package-scoped Clippy, and
   workspace Clippy.
4. Obtain dual independent implementation review and close RTR-033 only at the
   exact correction commit.
5. Rebuild the release planner and delegate one changed-head qualification.

## Exit Criteria

- Both named tests remain behavior-identical and below 100 lines.
- Focused tests and exact workspace Clippy pass.
- Package admission is `READY` with zero unauthorized paths.
- Dual independent review passes; RTR-033 is durably closed.
- One delegated changed-head qualification passes, followed by dual terminal
  verification without rerunning HEAVY.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two bounded implementation agents for separate test
modules, two independent read-only implementation reviewers, one comparator
runner for the exact qualification, and two independent read-only terminal
verifiers. Expected outputs are committed mechanical corrections, package-local
review/verification artifacts, and retained external qualification evidence.
Implementation write access is limited to the assigned test module; all review
and verification access is read-only. Do not push, deploy, switch branches,
manually dispatch TESTGATE, run HEAVY on the parent, or repeat unchanged gates.
