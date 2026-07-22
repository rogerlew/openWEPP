# CQR Nightly B02-2: Verifier Coverage Test Mutation

Package: `20260722-cqr-nightly-b02-2-verifier-coverage-tests-001`
Status: `ACTIVE`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `b02`
Target module: `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
Target rank: `2` of `2`
Quality dimension: `CRAP/cyclomatic-complexity`
Aggregate admission package: `docs/work-packages/20260722-cqr-nightly-b02-aggregate-001/package.md`
Aggregate scaffold commit: `ddd0e4aae924b7d9d8eca91b377106676c4d4dcf`
Aggregate batch manifest: `docs/work-packages/20260722-cqr-nightly-b02-aggregate-001/artifacts/batch-authority.json`
Master ExecPlan: `docs/work-packages/20260722-cqr-nightly-b02-execplan.md`

## Objective

Characterize and behavior-preservingly decompose `replace_string` from CRAP 56
to at most 30 without changing recursive traversal, value-only mutation, object
keys, scalar handling, match equality, or consumer fixture behavior.

## Required Reading

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, comparator, and closure-runner
subagents for CQR review, focused metric verification, and terminal evidence.
Expected outputs are package-local artifacts and retained external logs. Write
access is read-only unless explicitly assigned inside the declared module path.

## Scope

In scope: direct characterization and private whole-arm/container extraction for
`replace_string`. Out of scope: changed recursion, mutation semantics, key
rewrites, verifier behavior, production code, public APIs, and unrelated test
cleanup.

## Intended Write Set

- `crates/openwepp-gate-planner/src/verifier_coverage_tests.rs`
- `docs/work-packages/20260722-cqr-nightly-b02-2-verifier-coverage-tests-001/**`
- `docs/work-packages/README.md`

## Phase Plan

1. Retain aggregate-admission PASS and baseline classification.
2. Add direct characterization for root, nested, repeated, nonmatching,
   scalar/null, key, empty-container, and idempotence behavior.
3. Extract string, array, and object traversal without reordering recursion.
4. Measure target CRAP, run focused gates, dual review, and dual verification.

## Exit Criteria

- The target and all extracted helpers are CRAP at most 30.
- Recursive mutation behavior and fixture consumers remain unchanged.
- Focused characterization, formatting, Clippy, aggregate admission, dual
  review, and dual verification pass.
