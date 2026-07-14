# CQR Nightly Package Template

Copy this template to:

`docs/work-packages/YYYYMMDD-cqr-nightly-<batch-prefix><rank>-<module-slug>-001/package.md`

Also copy `docs/work-packages/templates/cqr-nightly-kickoff-prompt.md` into
`prompts/active/` for the package kickoff prompt. Then replace every
`{{placeholder}}` before executing.

## Package

Package: `{{package_id}}`
Status: `QUEUED`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `{{batch_ordinal}}`
Target module: `{{target_module_path}}`
Target rank: `{{rank}}` of `{{selected_count}}`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in `{{target_module_path}}` with CRAP
above `30` to `<= 30`, or record an ADR-0021-style disposition when a row is not
safely reducible as behavior-preserving CQR. Preserve runtime behavior and output
identity for all existing valid inputs.

Eligibility is symbol-level and defaults to eligible. The package must preserve
raw CRAP rows and separately record the exact `E-*`, `R-*`, or `X-*`
classification for every target row above 30; filenames and module-wide globs
cannot grant exclusions.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `{{target_module_path}}`
- `{{focused_tests_or_contracts}}`

If the target touches science-contract authority, contract tests, or
conservation-sensitive outputs, also read the nearest relevant `SC-*` contract
and `docs/specifications/science-contracts/AGENTS.md` before editing.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and output-identity checks. Expected outputs are
package-local `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
compact metrics, command logs, and artifact paths. Write access is read-only
unless a subagent is explicitly assigned a bounded implementation fix in
`{{target_module_path}}` or package-local artifacts.

Subagent requirement: this package requires spawning `comparator_suite_runner`
for heavy batch/closure/comparator runs, including full-workspace CRAP/coverage
after implementation, `cargo nextest run --workspace --profile full`, comparator
suites, and population/fixture batches. Do not run those heavy gates locally on
the parent model unless the subagent is unavailable; if unavailable, record
command-level evidence before running locally.

## Scope

In scope:

- characterization tests required to make the target safe to refactor;
- behavior-preserving helper extraction or control-flow simplification inside
  `{{target_module_path}}`;
- local private helpers needed by the extraction;
- package artifacts and prompt material;
- focused tests that prove existing behavior.

Out of scope:

- science-formula changes;
- threshold, tolerance, or contract changes;
- public output semantics or serialization changes;
- fail-closed behavior changes;
- opportunistic cleanup outside the target module;
- unrelated active package work.

## Intended Write Set

- `{{target_module_path}}`
- `{{test_paths}}`
- `docs/work-packages/{{package_id}}/**`
- `docs/work-packages/README.md` after closure if catalog update is needed

Do not edit unrelated dirty files. If the target path is already dirty from
unrelated work, stop before implementation and record a global/process hold.

## Scaffold Commit Gate

Before any production/test implementation edit, commit the scaffold for this
package. The scaffold commit includes this `package.md`, prompt directories,
artifact placeholders including `artifacts/eligibility-classification.md`,
target-selection evidence, baseline command provenance, and this gate list.

If the scaffold commit cannot be created because commits are not authorized, stop
and report the blocked commit boundary.

## Phase Plan

### Phase A - Baseline

1. Record `git status --short --branch`.
2. Record the target-selection row, raw/actionable counts, and reviewed
   classifications in
   `artifacts/target-selection.md`.
3. Populate `artifacts/eligibility-classification.md` with each raw row's exact
   file/function/line, source SHA-256, CRAP/CC/coverage, ADR-0021 class,
   aggregate/floor/CRAP treatment, evidence, and proposed reviewer disposition.
   Hand-authored parser, guard, error-precedence, state/order/key, numerical,
   serialization/publication, and consumer behavior remains eligible.
4. Populate `artifacts/required-reading-map.md` with path, tier, rationale,
   applicability trigger, and read status for all kickoff required reading.
5. Run or copy from the batch measurement:
   - `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path {{lcov_before_path}}`
   - `cargo crap --workspace --lcov {{lcov_before_path}} --min 0 --format json --output {{crap_before_path}}`
6. Summarize raw and actionable target rows separately in
   `artifacts/crap-before.md` and
   `artifacts/coverage-before.md`.

### Phase B - Characterization

1. Identify current focused tests for `{{target_module_path}}`.
2. Add characterization tests before refactoring when existing coverage is
   insufficient.
3. If characterization tests are added or materially changed, record ADR-0021
   coverage closure in `artifacts/coverage-closure.md`:
   - tier assignment (`science` or `glue`);
   - line and region threshold status;
   - per-function 75% region-floor status or disposition;
   - applicable obligation-to-test binding, including `SC-*` obligations when
     contract-derived behavior is covered.
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
   - `bash tools/release/run_adjudicated_crap_gate.sh --base-ref <frozen-base>`
6. Use `comparator_suite_runner` for heavy full-workspace/batch gates when
   available. If unavailable, record the spawn/tool-policy failure before
   running locally.

### Phase E - Review, Verification, Disposition

1. Complete dual review with findings ordered by severity.
2. Require both reviewers to accept every `R-OBSERVABILITY`,
   `R-IRREDUCIBLE-CRAP`, or `X-*` disposition before it can leave the actionable
   set; an unaccepted row remains eligible and blocks closure or requires
   implementation. `R-INFRASTRUCTURE` cannot waive CRAP above 30.
3. Disposition every finding as `accepted`, `rejected`, `deferred`, or
   `follow-up`.
4. Complete dual verification against this package and the nightly ExecPlan.
5. Update `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
   `artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
   `artifacts/disposition.md`, `artifacts/final-disposition.md`, and
   `artifacts/worker-handoff.md`.
6. Commit completion or hold evidence before starting the next nightly CQR
   package.

## Hold Rules

Local target holds roll back only in-progress production/test implementation
edits to the scaffold baseline, preserve package evidence, commit the hold
package, and allow the nightly batch to continue to the next selected module.

Global/process holds stop the nightly batch. Examples include red baseline gates,
unavailable CRAP tooling, dirty overlap with active work, broad output-identity
breaks, and inability to create the required scaffold/completion commits.
Rollback only current-package implementation edits and preserve package evidence;
do not revert unrelated user changes or active-package work.

Hold packages must include `artifacts/hold-legitimacy-audit.md` naming the exact
blocker, evidence, attempted in-envelope route, why CQR cannot close it safely,
rollback proof, and first actionable follow-on.

## Exit Criteria

Complete only when:

- scaffold commit exists before implementation edits;
- active kickoff prompt exists and includes `Execution mode`, `Autonomy`, tiered
  required reading, required-reading budget/map, and required subagent wording;
- every raw row above 30 has an exact eligibility classification and both raw
  and actionable counts are recorded;
- every `E-*` and `R-INFRASTRUCTURE` target function is `<= 30` CRAP; every
  removed `R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` row has dual-review
  acceptance and exact evidence;
- ADR-0021 coverage closure is recorded when characterization tests are added or
  materially changed;
- behavior identity is proven by focused tests and appropriate output/API
  evidence;
- no current-scope gate is deferred without hold classification;
- dual review findings are dispositioned and accepted findings are fixed;
- dual verification passes;
- completion or hold commit exists before the next target package starts.

Final status options:

- `EXECUTED-COMPLETE-CQR-NIGHTLY`
- `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-*`
- `EXECUTED-HOLD-CQR-NIGHTLY-GLOBAL-*`
