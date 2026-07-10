# CQR Nightly Batch 02, Target 10 — HBP Payload Validator

Package: `20260709-cqr-nightly-b02-10-hbp-payload-validator-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module:
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
Target rank: `10` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs` with CRAP
above `30` to `<= 30`, or record an ADR-0021-style disposition when a row is
not safely reducible as behavior-preserving CQR. Preserve HBP payload schema,
binary decoding, typed errors, guard semantics, and public parser behavior.

## Scope

In scope:

- characterization tests required to make payload validator decomposition safe;
- behavior-preserving helper extraction or control-flow simplification inside
  `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`;
- package artifacts and prompt material;
- focused tests that prove existing HBP parser and payload validator behavior.

Out of scope:

- HBP binary format, schema, event semantics, state-snapshot semantics, guard
  IDs, thresholds, units, or public API changes;
- science-formula, contract-authority, or runtime publication changes;
- opportunistic cleanup outside the target module.

## Intended Write Set

- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
- `tests/integration/infile_hbp_parser_contract.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/**`
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
- `docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and nearest relevant
  `SC-*` contract only if the change touches contract authority,
  conservation-sensitive outputs, or contract-derived tests.
- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need to
  be narrowed before the final closure loop.

On-demand:

- `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`
- `tests/integration/infile_hbp_parser_contract.rs`
- adjacent HBP parser modules imported by the payload validator

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and output-identity checks. Expected outputs are
package-local `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
compact metrics, command logs, and artifact paths. Write access is read-only
unless a subagent is explicitly assigned a bounded implementation fix in
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`,
`tests/integration/infile_hbp_parser_contract.rs`, or package-local artifacts.

Subagent requirement: this package requires spawning `comparator_suite_runner`
for heavy batch/closure/comparator runs, including full-workspace CRAP/coverage
after implementation, `cargo nextest run --workspace --profile full`,
comparator suites, and population/fixture batches. Do not run those heavy gates
locally on the parent model unless the subagent is unavailable; if unavailable,
record command-level evidence before running locally.

## Required Gates

Commit scaffold before edits; use test-first characterization, delegated heavy
metrics/gates, dual review/verification, finding disposition, line-count
governance, and completion or hold commit before ending batch 02.

## Phase Plan

### Phase A - Baseline

1. Record `git status --short --branch`.
2. Record target-selection row and exclusions in
   `artifacts/target-selection.md`.
3. Populate `artifacts/required-reading-map.md`.
4. Use the batch baseline:
   - LCOV: `/tmp/openwepp-cqr-nightly-new-isolated.lcov`
   - CRAP JSON: `/tmp/openwepp-cqr-nightly-new-isolated-crap.json`
5. Summarize target rows in `artifacts/crap-before.md` and
   `artifacts/coverage-before.md`.

### Phase B - Characterization

1. Identify current focused tests for HBP payload validation.
2. Add characterization tests before refactoring when existing coverage is
   insufficient.
3. If characterization tests are added or materially changed, record ADR-0021
   coverage closure in `artifacts/coverage-closure.md`.
4. Record behavior oracle and command evidence in `artifacts/characterization.md`.

### Phase C - Refactor

1. Extract cohesive branch/guard/loop blocks one at a time.
2. Preserve byte-read order, cursor consumption, short-circuit behavior, typed
   error behavior, schema semantics, and numeric scaling.
3. Run focused tests after each meaningful extraction.
4. Stop editing when every owned production function is `<= 30` or explicitly
   dispositioned.
5. Record implementation in `artifacts/implementation.md`.

### Phase D - Metrics And Gates

1. Re-run coverage and CRAP with the same method as Phase A or record a
   package-approved focused target equivalent if full coverage is unavailable.
2. Record after metrics, numeric/API/output identity, line-count governance, and
   exact gate results.
3. Run and record diff whitespace, docs lint, focused tests, fmt, workspace
   clippy, full nextest, and deny.

### Phase E - Review, Verification, Disposition

1. Complete dual review and finding disposition.
2. Complete dual verification against this package and the nightly ExecPlan.
3. Update final disposition and worker handoff.
4. Commit completion or hold evidence before ending batch 02.

## Hold Rules

Local target holds roll back only in-progress production/test implementation
edits to the scaffold baseline, preserve package evidence, commit the hold
package, and allow the nightly batch to close with a held target.

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
- completion or hold commit exists before ending batch 02.
