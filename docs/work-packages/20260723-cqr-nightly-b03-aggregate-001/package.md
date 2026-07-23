# CQR Nightly B03 Aggregate Admission

Package ID: `20260723-cqr-nightly-b03-aggregate-001`

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-INVALID-AGGREGATE-SCAFFOLD`

## Objective

Provide prospective aggregate authority for the two-module CQR batch exposed
by TESTGATE receipt `64a6f292...26b44`: one module package for `main.rs` and one
for `package_validation.rs`. Preserve behavior while reducing every actionable
row to CRAP at most 30, then return to one changed-head recovery qualification.

## Intended Write Set

- `crates/openwepp-gate-planner/src/main.rs`
- `crates/openwepp-gate-planner/src/package_validation.rs`
- `docs/work-packages/20260723-cqr-nightly-b03-aggregate-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03-1-main-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03-2-package-validation-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`
- `docs/work-packages/20260722-testgate-sequential-package-authority-recovery-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/README.md`

## Execution Contract

The immutable batch manifest enumerates the master ExecPlan, both module
packages, source paths, catalog, and closeout evidence. Each module scaffold
must be committed after this aggregate scaffold and pass canonical aggregate
admission before implementation edits.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewers, two terminal verifiers, and
one comparator runner. Expected outputs are package-local artifacts and
retained external evidence. Write access is read-only except for an explicitly
assigned bounded module implementation or comparator artifact root.

## Exit Criteria

- Both module packages complete their exact CRAP and behavior gates.
- Every review finding is dispositioned and dual terminal verification passes.
- The final changed-head recovery qualification passes without an unchanged
  retry.

## Disposition

Ran: HOLD before implementation. Canonical aggregate admission rejected this
immutable scaffold because it declared `Intended Write Set` instead of the
required `Declared Write Set`. A new prospective aggregate must replace it;
this package's write set and batch manifest are not amended retroactively.
