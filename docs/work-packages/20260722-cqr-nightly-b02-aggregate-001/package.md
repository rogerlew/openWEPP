# CQR Nightly B02 Aggregate Admission

Package ID: `20260722-cqr-nightly-b02-aggregate-001`

Status: `ACTIVE`

## Objective

Provide prospective aggregate authority for the two-module CQR batch exposed by
TESTGATE receipt `cea13649...cd5ce`: one module package for `executor.rs` and
one for `verifier_coverage_tests.rs`. Preserve behavior while reducing every
actionable row to CRAP at most 30, then return to one changed-head recovery
qualification.

## Declared Write Set

- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/executor_coverage_tests.rs`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
- `docs/work-packages/20260722-cqr-nightly-b02-aggregate-001/**`
- `docs/work-packages/20260722-cqr-nightly-b02-1-executor-001/**`
- `docs/work-packages/20260722-cqr-nightly-b02-2-verifier-coverage-tests-001/**`
- `docs/work-packages/20260722-cqr-nightly-b02-execplan.md`
- `docs/work-packages/20260722-testgate-affected-crap-authority-reexpression-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Execution Contract

The immutable batch manifest enumerates the master ExecPlan, both module
packages, source/test paths, and closeout evidence. Each module scaffold must be
committed after this aggregate scaffold and pass the canonical aggregate
admission validator before implementation edits.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers, two read-only
terminal verifiers, and one comparator runner for final TESTGATE qualification.
Expected outputs are package-local review/verification artifacts and retained
external gate evidence. Write access is read-only except for the comparator's
ignored artifact root. Do not push, deploy, switch branches, manually dispatch
TESTGATE, run HEAVY on the parent, or repeat unchanged expensive gates.
