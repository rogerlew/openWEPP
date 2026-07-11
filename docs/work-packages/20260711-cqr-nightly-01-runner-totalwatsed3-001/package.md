# CQR Nightly Batch 01, Target 01 — Runner Totalwatsed3

Package: `20260711-cqr-nightly-01-runner-totalwatsed3-001`
Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-COVERAGE-PRECONDITION`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `01`
Target module: `crates/openwepp-runner/src/totalwatsed3.rs`
Target rank: `1` of `8`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in
`crates/openwepp-runner/src/totalwatsed3.rs` with CRAP above `30` to `<= 30`,
or record an ADR-0021-style disposition when a row is not safely reducible as
behavior-preserving CQR. Preserve totalwatsed3 aggregation, typed errors,
Parquet schema, units, row identity, and numeric output exactly.

## Required Reading

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- this package and `artifacts/required-reading-map.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and the nearest relevant
  `SC-*` contract if implementation would touch formula, conservation, or
  publication authority. Such a semantic change is outside this package.
- `docs/standards/local-ci-gate-selection.md` when narrowing iteration gates.

On-demand: adjacent runner/output modules used by the target.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and output-identity checks. Expected outputs are
package-local review/verification artifacts, compact metrics, command logs, and
artifact paths. Write access is read-only unless a subagent is explicitly
assigned a bounded fix in the intended write set.

Subagent requirement: this package requires spawning `comparator_suite_runner`
for heavy batch/closure/comparator runs, including full-workspace CRAP/coverage,
`cargo nextest run --workspace --profile full`, comparator suites, and fixture
batches. Do not run those heavy gates on the parent model unless the subagent is
unavailable; record command-level evidence before any local substitution.

## Scope And Intended Write Set

In scope: characterization tests; behavior-preserving private helper extraction
or control-flow decomposition in the target; package artifacts and prompt.

Out of scope: formula, unit, threshold, schema, serialization, public-output,
fail-closed, or public-API changes; unrelated cleanup.

Write set:

- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`
- `docs/work-packages/20260711-cqr-nightly-01-runner-totalwatsed3-001/**`
- `docs/work-packages/README.md` at closure

## Scaffold Commit Gate

Commit this scaffold before production/test implementation edits. If the
commit cannot be created, stop the batch as a global/process hold.

## Phase Plan

1. Baseline: record selection, reading, CRAP, coverage, and characterization.
2. Cover first: add tests only when existing behavior is under-characterized;
   record ADR-0021 closure when tests materially change.
3. Refactor: extract cohesive blocks while preserving statement, floating-point,
   accumulation, short-circuit, row-read, and typed-error order.
4. Re-measure target coverage/CRAP and prove output identity.
5. Run diff/docs/focused/fmt plus delegated clippy/full-nextest/deny gates.
6. Complete dual review, finding disposition, dual verification, final
   disposition, handoff, and completion/hold commit.

## Hold Rules

Local target holds roll back only implementation/test edits to this scaffold,
preserve and commit hold evidence, and permit target 02 to start. Global/process
holds stop the batch. Never revert unrelated work.

## Exit Criteria

- Scaffold commit predates implementation/test edits.
- Every target production function is CRAP `<= 30` or legitimately dispositioned.
- Added/materially changed tests satisfy ADR-0021 coverage closure.
- Numeric/output/API identity and line-count governance are recorded.
- Every required gate has direct current evidence; no failed/deferred gate is
  classified complete.
- Dual reviews and verifications pass with all findings dispositioned.
- Completion or hold evidence is committed before target 02 begins.
