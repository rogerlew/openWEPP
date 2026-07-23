# CQR Nightly B03R Aggregate Admission

Package ID: `20260723-cqr-nightly-b03r-aggregate-001`

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-INCOMPLETE-BATCH-MANIFEST`

## Objective

Provide corrected prospective aggregate authority for the two-module CQR batch
exposed by TESTGATE receipt `64a6f292...26b44`. This scaffold replaces the
terminal pre-implementation B03 scaffold and uses the exact immutable aggregate
contract required by canonical admission.

## Declared Write Set

- `crates/openwepp-gate-planner/src/main.rs`
- `crates/openwepp-gate-planner/src/package_validation.rs`
- `docs/work-packages/20260723-cqr-nightly-b03r-aggregate-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03r-1-main-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03r-2-package-validation-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`
- `docs/work-packages/20260722-testgate-sequential-package-authority-recovery-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Execution Contract

The immutable batch manifest enumerates the master ExecPlan, both corrected
module packages, source paths, catalog, and closeout evidence. Each module
scaffold must be committed after this aggregate and pass canonical aggregate
admission before implementation edits.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewers, two terminal verifiers, and
one comparator runner. Expected outputs are package-local artifacts and
retained external evidence. Write access is read-only except for an explicitly
assigned bounded module implementation or comparator artifact root.

## Exit Criteria

- Both corrected module packages complete exact CRAP and behavior gates.
- Every review finding is dispositioned and dual terminal verification passes.
- One changed-head recovery qualification passes without an unchanged retry.

## Disposition

Ran: HOLD before implementation. Canonical aggregate admission required exact
module package paths in `required_paths`; the immutable manifest contained only
their recursive package paths. A new prospective aggregate replaces it.
