# CQR Nightly Batch 02, Target 09 - Slope Parser

Package: `20260709-cqr-nightly-b02-09-slope-parser-001`
Status: `SCAFFOLDED-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-input-contract/src/parsers/slope.rs`
Target rank: `9` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in
`crates/openwepp-input-contract/src/parsers/slope.rs` with CRAP above `30` to
`<= 30`, or record an ADR-0021-style disposition when a row is not safely
reducible as behavior-preserving CQR. Preserve parser behavior, typed errors,
guard IDs, file grammar, numeric thresholds, and public API semantics.

## Scope

In scope:

- characterization tests required to make slope parser decomposition safe;
- behavior-preserving helper extraction or control-flow simplification inside
  `crates/openwepp-input-contract/src/parsers/slope.rs`;
- package artifacts and prompt material;
- focused tests that prove existing slope parser behavior.

Out of scope:

- public parser API, enum variant, grammar, guard ID, tolerance, or numeric
  threshold changes;
- science-formula, contract-authority, or runtime publication changes;
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
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`
- `tests/integration/infile_watershed_structure_parser_contract.rs`

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
`tests/integration/infile_slope_parser_contract.rs`, or package-local artifacts.

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
