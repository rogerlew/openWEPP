# CQR Nightly 08 - Runner Errors

Package: `20260709-cqr-nightly-08-runner-errors-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Target module:
`crates/openwepp-runner/src/errors.rs`
Target rank: `8` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in
`crates/openwepp-runner/src/errors.rs` with CRAP above `30` to `<= 30`, or
record an ADR-0021-style disposition when a row is not safely reducible as
behavior-preserving CQR. Preserve runner error taxonomy, stable error codes,
source chaining, public error APIs, CLI-visible message meaning, release
metadata/lint failure semantics, and hillslope CLI error behavior.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `crates/openwepp-runner/src/errors.rs`
- `tests/integration/cli01_runner_contract_derived_tests.rs`
- `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/package.md`
- `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/artifacts/required-reading-map.md`

This package is glue-tier runner error CQR. It must not amend release contracts,
CLI behavior, error-code meanings, source-chain ownership, launch behavior,
sidecar behavior, or public output semantics. If a row cannot be closed without
changing those semantics, hold rather than changing authority.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and output-identity checks. Expected outputs are
package-local `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
compact metrics, command logs, and artifact paths. Write access is read-only
unless a subagent is explicitly assigned a bounded implementation fix in
`crates/openwepp-runner/src/errors.rs`,
`tests/integration/cli01_runner_contract_derived_tests.rs`, or package-local
artifacts.

Subagent requirement: this package requires spawning `comparator_suite_runner`
for heavy batch/closure/comparator runs, including full-workspace CRAP/coverage
after implementation, `cargo nextest run --workspace --profile full`, comparator
suites, and population/fixture batches. Do not run those heavy gates locally on
the parent model unless the subagent is unavailable; if unavailable, record
command-level evidence before running locally.

## Scope

In scope:

- characterization tests required to make the runner error module safe to
  refactor;
- behavior-preserving helper extraction or control-flow simplification inside
  the target module;
- local private helpers needed by the extraction;
- focused runner integration tests that prove stable error codes, display text,
  and source chaining;
- package artifacts and prompt material.

Out of scope:

- error-code changes;
- CLI-visible error meaning changes;
- source-chain ownership changes;
- release metadata/lint contract changes;
- launch, sidecar, runtime, or output behavior changes;
- public error API changes;
- serialization or public output semantic changes;
- broad fixture rewrites or unrelated runner cleanup;
- unrelated active package work.

## Intended Write Set

- `crates/openwepp-runner/src/errors.rs`
- `tests/integration/cli01_runner_contract_derived_tests.rs`
- package-local artifacts under
  `docs/work-packages/20260709-cqr-nightly-08-runner-errors-001/**`
- `docs/work-packages/README.md` after closure if catalog update is needed

Do not edit unrelated dirty files. If the target path is already dirty from
unrelated work, stop before implementation and record a global/process hold.

Line-count note: the target starts at `549` lines, below the 2000-line WARN
threshold and below the 3000-line blocker.

## Scaffold Commit Gate

Before any production/test implementation edit, commit the scaffold for this
package. The scaffold commit includes this `package.md`, prompt directories,
artifact placeholders, target-selection evidence, baseline command provenance,
and this gate list.

If the scaffold commit cannot be created because commits are not authorized,
stop and report the blocked commit boundary.

## Phase Plan

### Phase A - Baseline

1. Record `git status --short --branch`.
2. Record the target-selection row and exclusions in
   `artifacts/target-selection.md`.
3. Populate `artifacts/required-reading-map.md` with path, tier, rationale,
   applicability trigger, and read status for all kickoff required reading.
4. Copy baseline metrics from the live nightly measurement:
   - `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov`
   - `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json`
5. Summarize target rows in `artifacts/crap-before.md` and
   `artifacts/coverage-before.md`.

### Phase B - Characterization

1. Identify current focused tests for the target module.
2. Add characterization tests before refactoring when existing coverage is
   insufficient.
3. If characterization tests are added or materially changed, record ADR-0021
   coverage closure in `artifacts/coverage-closure.md`:
   - tier assignment (`glue`);
   - line and region threshold status;
   - per-function 75% region-floor status or disposition;
   - applicable obligation-to-test binding for runner error behavior without
     creating new authority.
4. Record the behavior oracle and command evidence in
   `artifacts/characterization.md`.

### Phase C - Refactor

1. Extract cohesive display/code/source branch clusters one at a time.
2. Preserve stable error codes, display wording, source chaining, and
   short-circuit behavior.
3. Run focused tests after each meaningful extraction.
4. Stop editing when every owned production function is `<= 30` or explicitly
   dispositioned.
5. Record the implementation in `artifacts/implementation.md`.

### Phase D - Metrics And Gates

1. Re-run coverage and CRAP with the same method as Phase A or a documented
   targeted equivalent when full-workspace coverage is blocked by unrelated
   coverage-instrumented tests.
2. Record `artifacts/crap-after.md` and `artifacts/coverage-after.md`.
3. Record behavior/API/output identity in `artifacts/numeric-equivalence.md`.
4. Record `.rs` line-count governance in
   `artifacts/line-count-governance.md`.
5. Run and record:
   - `git diff --check`
   - markdown/doc lint for touched docs
   - focused runner tests
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo nextest run --workspace --profile full`
   - `cargo deny check`
6. Use `comparator_suite_runner` for heavy full-workspace/batch gates when
   available. If unavailable, record the spawn/tool-policy failure before
   running locally.

### Phase E - Review, Verification, Disposition

1. Complete dual review with findings ordered by severity.
2. Disposition every finding as `accepted`, `rejected`, `deferred`, or
   `follow-up`.
3. Complete dual verification against this package and the nightly ExecPlan.
4. Update `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
   `artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
   `artifacts/disposition.md`, `artifacts/final-disposition.md`, and
   `artifacts/worker-handoff.md`.
5. Commit completion or hold evidence before starting the next nightly CQR
   package.

## Hold Rules

Local target holds roll back only in-progress production/test implementation
edits to the scaffold baseline, preserve package evidence, commit the hold
package, and allow the nightly batch to continue to the next selected module.

Global/process holds stop the nightly batch. Examples include red baseline
gates, unavailable CRAP tooling, dirty overlap with active work, broad
output-identity breaks, target-file crossing the 3000-line blocker without a
valid refactor, and inability to create required scaffold/completion commits.
Rollback only current-package implementation edits and preserve package
evidence; do not revert unrelated user changes or active-package work.

Hold packages must include `artifacts/hold-legitimacy-audit.md` naming the exact
blocker, evidence, attempted in-envelope route, why CQR cannot close it safely,
rollback proof, and first actionable follow-on.

## Exit Criteria

Complete only when:

- scaffold commit exists before implementation edits;
- active kickoff prompt exists and includes `Execution mode`, `Autonomy`, tiered
  required reading, required-reading budget/map, and required subagent wording;
- every target production function is `<= 30` CRAP or dispositioned;
- ADR-0021 coverage closure is recorded when characterization tests are added or
  materially changed;
- behavior identity is proven by focused tests and appropriate runner
  error/API evidence;
- no current-scope gate is deferred without hold classification;
- dual review findings are dispositioned and accepted findings are fixed;
- dual verification passes;
- completion or hold commit exists before the next target package starts.

Final status options:

- `EXECUTED-COMPLETE-CQR-NIGHTLY`
- `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-*`
- `EXECUTED-HOLD-CQR-NIGHTLY-GLOBAL-*`
