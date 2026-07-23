# CQR Nightly B03S-2: Package Validation Entry Point

Package: `20260723-cqr-nightly-b03s-2-package-validation-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

Completion commit: `c85c1a15d9b8fdd63f328a125bde345f898ad444`
ExecPlan: `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`
Nightly batch: `b03s`
Target module: `crates/openwepp-gate-planner/src/package_validation.rs`
Target rank: `2` of `2`
Quality dimension: `CRAP/cyclomatic-complexity`
Aggregate admission package: `docs/work-packages/20260723-cqr-nightly-b03s-aggregate-001/package.md`
Aggregate scaffold commit: `f72e59d0917940a611378515219d6f44d6ef5604`
Aggregate batch manifest: `docs/work-packages/20260723-cqr-nightly-b03s-aggregate-001/artifacts/batch-authority.json`
Master ExecPlan: `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`

## Objective

Characterize and behavior-preservingly reduce `validate_package` from CRAP 156
to at most 30 without changing Git evidence collection, status/reason codes,
write-set matching, or audit identity.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `crates/openwepp-gate-planner/src/package_validation.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two reviewers, two verifiers, and comparator/metric
runners. Outputs are package-local artifacts and retained external evidence.
Write access is read-only unless explicitly assigned within the target module
or package artifacts.

## Scope

In scope: characterization and private whole-stage/guard extraction inside
`package_validation.rs`. Out of scope: authority semantics, accepted status
grammar, reason codes, Git traversal order, hashing, schemas, thresholds, and
unrelated cleanup.

## Intended Write Set

- `crates/openwepp-gate-planner/src/package_validation.rs`
- `docs/work-packages/20260723-cqr-nightly-b03s-2-package-validation-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Retain aggregate-admission PASS and exact baseline classification.
2. Bind direct characterization to success and every existing rejection seam.
3. Extract whole validation stages without reordering or changing evidence.
4. Re-measure CRAP, run focused gates, dual review, and dual verification.

## Exit Criteria

- The target and every extracted helper has CRAP at most 30.
- Exact audit JSON, reason ordering, hashes, and status remain unchanged.
- Focused tests, formatting, Clippy, aggregate admission, dual review, and dual
  verification pass.

## Completion

Completed at exact commit `c85c1a15d9b8fdd63f328a125bde345f898ad444`.
Target/helper CRAP is 4–5 at 100% focused coverage. Focused gates, aggregate
admission, dual implementation review, and dual terminal verification passed.
Aggregate closeout owns the later changed-head TESTGATE qualification.
