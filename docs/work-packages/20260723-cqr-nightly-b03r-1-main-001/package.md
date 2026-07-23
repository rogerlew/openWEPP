# CQR Nightly B03R-1: Gate Planner Main Authority Commands

Package: `20260723-cqr-nightly-b03r-1-main-001`
Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-INCOMPLETE-BATCH-MANIFEST`
ExecPlan: `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`
Nightly batch: `b03r`
Target module: `crates/openwepp-gate-planner/src/main.rs`
Target rank: `1` of `2`
Quality dimension: `CRAP/cyclomatic-complexity`
Aggregate admission package: `docs/work-packages/20260723-cqr-nightly-b03r-aggregate-001/package.md`
Aggregate scaffold commit: `8cd657982892f6cb9548d098826df31ee44eeccb`
Aggregate batch manifest: `docs/work-packages/20260723-cqr-nightly-b03r-aggregate-001/artifacts/batch-authority.json`
Master ExecPlan: `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`

## Objective

Characterize and behavior-preservingly reduce
`validate_package_chain_command`, `plan_request`, and `package_authority` to
CRAP at most 30 without changing CLI parsing, typed errors, output persistence,
or independent authority reconstruction.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `crates/openwepp-gate-planner/src/main.rs`
- `crates/openwepp-gate-planner/src/package_validation.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two reviewers, two verifiers, and comparator/metric
runners. Outputs are package-local artifacts and retained external evidence.
Write access is read-only unless explicitly assigned within the target module
or package artifacts.

## Scope

In scope: characterization coverage and private whole-guard/request extraction
inside `main.rs`. Out of scope: public CLI changes, error/reason-code changes,
authority semantics, persistence changes, thresholds, and unrelated cleanup.

## Intended Write Set

- `crates/openwepp-gate-planner/src/main.rs`
- `docs/work-packages/20260723-cqr-nightly-b03r-1-main-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Retain aggregate-admission PASS and exact baseline classifications.
2. Add or bind direct characterization for every target branch.
3. Extract whole parsing/reconstruction/persistence guards without reordering.
4. Re-measure CRAP, run focused gates, dual review, and dual verification.

## Exit Criteria

- Each target and every extracted helper has CRAP at most 30.
- Exact CLI output, typed errors, persistence, and reconstruction remain
  unchanged.
- Focused tests, formatting, Clippy, aggregate admission, dual review, and dual
  verification pass.

## Hold Disposition

Ran: canonical aggregate admission failed before implementation because the
bound immutable manifest omitted exact mandatory module package paths. A new
prospectively bound module package owns execution.
