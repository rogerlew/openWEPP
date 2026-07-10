# CQR Nightly Batch 02, Target 09 — Slope Parser

Package: `20260709-cqr-nightly-b02-09-slope-parser-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-input-contract/src/parsers/slope.rs`
Target rank: `9` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in
`crates/openwepp-input-contract/src/parsers/slope.rs` with CRAP above `30` to
`<= 30`, or record an ADR-0021-style disposition when a row is not safely
reducible as behavior-preserving CQR. Preserve runtime behavior and output
identity for all existing valid inputs.

## Scope

In scope:

- characterization tests required to make the target safe to refactor;
- behavior-preserving helper extraction or control-flow simplification inside
  `crates/openwepp-input-contract/src/parsers/slope.rs`;
- package artifacts and prompt material;
- focused tests that prove existing slope parser behavior.

Out of scope:

- slope grammar, distance-mode, geometry, or typed error semantic changes;
- science-formula, threshold, tolerance, or contract-authority changes;
- serialization or public output semantic changes;
- fail-closed behavior changes;
- opportunistic cleanup outside the target module.

## Intended Write Set

- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `tests/integration/infile_slope_parser_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001/**`
- `docs/work-packages/README.md` after closure if catalog update is needed

Do not edit unrelated dirty files. If a declared write-set path is already dirty
from unrelated work, stop before implementation and record a global/process
hold.

## Required Reading

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and nearest relevant
  `SC-*` contract only if the change touches contract authority,
  conservation-sensitive outputs, or contract-derived tests.
- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need to
  be narrowed before the final closure loop.

On-demand:

- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `tests/integration/infile_slope_parser_contract.rs`
- adjacent parsers imported by the slope parser

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and output-identity checks. Expected outputs are
package-local `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
compact metrics, command logs, and artifact paths. Write access is read-only
unless a subagent is explicitly assigned a bounded implementation fix in
`crates/openwepp-input-contract/src/parsers/slope.rs`,
`tests/integration/infile_slope_parser_contract.rs`, or package-local
artifacts.

Subagent requirement: this package requires spawning `comparator_suite_runner`
for heavy batch/closure/comparator runs, including full-workspace CRAP/coverage
after implementation, `cargo nextest run --workspace --profile full`,
comparator suites, and population/fixture batches. Do not run those heavy gates
locally on the parent model unless the subagent is unavailable; if unavailable,
record command-level evidence before running locally.

## Required Gates

Commit scaffold before edits; use test-first characterization, delegated heavy
metrics/gates, dual review/verification, finding disposition, line-count
governance, and completion or hold commit before target 10.

## Phase Plan

### Phase A - Baseline

1. Record `git status --short --branch`.
2. Record the target-selection row and exclusions in
   `artifacts/target-selection.md`.
3. Populate `artifacts/required-reading-map.md` with path, tier, rationale,
   applicability trigger, and read status for kickoff required reading.
4. Use the batch baseline:
   - LCOV: `/tmp/openwepp-cqr-nightly-new-isolated.lcov`
   - CRAP JSON: `/tmp/openwepp-cqr-nightly-new-isolated-crap.json`
5. Summarize target rows in `artifacts/crap-before.md` and
   `artifacts/coverage-before.md`.

### Phase B - Characterization

1. Identify current focused tests for the slope parser.
2. Add characterization tests before refactoring when existing coverage is
   insufficient.
3. If characterization tests are added or materially changed, record ADR-0021
   coverage closure in `artifacts/coverage-closure.md`.
4. Record the behavior oracle and command evidence in
   `artifacts/characterization.md`.

### Phase C - Refactor

1. Extract cohesive branch/guard/loop blocks one at a time.
2. Preserve statement order, floating-point expression grouping, accumulation
   order, short-circuit behavior, and typed error behavior.
3. Run focused tests after each meaningful extraction.
4. Stop editing when every owned production function is `<= 30` or explicitly
   dispositioned.
5. Record the implementation in `artifacts/implementation.md`.

### Phase D - Metrics And Gates

1. Re-run coverage and CRAP with the same method as Phase A.
2. Record `artifacts/crap-after.md` and `artifacts/coverage-after.md`.
3. Record numeric/API/output identity in `artifacts/numeric-equivalence.md`.
4. Record `.rs` line-count governance in
   `artifacts/line-count-governance.md`.
5. Run and record:
   - `git diff --check`
   - markdown/doc lint for touched docs
   - focused tests for the touched module/crate
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo nextest run --workspace --profile full`
   - `cargo deny check`

### Phase E - Review, Verification, Disposition

1. Complete dual review with findings ordered by severity.
2. Disposition every finding as `accepted`, `rejected`, `deferred`, or
   `follow-up`.
3. Complete dual verification against this package and the nightly ExecPlan.
4. Update package artifacts through final disposition and worker handoff.
5. Commit completion or hold evidence before starting target 10.

## Hold Rules

Local target holds roll back only in-progress production/test implementation
edits to the scaffold baseline, preserve package evidence, commit the hold
package, and allow the nightly batch to continue to target 10.

Global/process holds stop the nightly batch. Examples include red baseline
gates, unavailable CRAP tooling, dirty overlap with active work, broad
output-identity breaks, and inability to create the required scaffold/completion
commits.

## Exit Criteria

Complete only when:

- scaffold commit exists before implementation edits;
- active kickoff prompt exists and includes execution mode, autonomy, tiered
  required reading, required-reading budget/map, and required subagent wording;
- every target production function is `<= 30` CRAP or dispositioned;
- ADR-0021 coverage closure is recorded when characterization tests are added or
  materially changed;
- behavior identity is proven by focused tests and appropriate output/API
  evidence;
- no current-scope gate is deferred without hold classification;
- dual review findings are dispositioned and accepted findings are fixed;
- dual verification passes;
- completion or hold commit exists before the next target package starts.
