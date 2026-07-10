# CQR Nightly 10 - Runner Laned Shadow

Package: `20260709-cqr-nightly-10-runner-laned-shadow-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Target module:
`crates/openwepp-runner/src/hillslope/laned_shadow.rs`
Target rank: `10` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in
`crates/openwepp-runner/src/hillslope/laned_shadow.rs` with CRAP above `30` to
`<= 30`, or record an ADR-0021-style disposition when a row is not safely
reducible as behavior-preserving CQR. Preserve Lane D diagnostic shadow
semantics, protected output byte identity, manifest diagnostic meaning,
active/shadow mutual exclusion, fail-closed dynamic operand validation, routing
coefficient authority, and routing coefficient consumption.

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
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`
- `tests/integration/laned_shadow_h2637.rs`
- `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/package.md`
- `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/artifacts/required-reading-map.md`

This package is science-sensitive diagnostic-shadow CQR. It must not change
published process authority, Lane D activation policy, routing equations,
coefficient source authority, fail-closed operand guards, diagnostic manifest
semantics, or protected public output meaning. If a row cannot be closed without
changing those semantics, hold rather than changing authority.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and numeric/output-identity checks. Expected
outputs are package-local `artifacts/review_agent_a.md`,
`artifacts/review_agent_b.md`, `artifacts/verification_agent_a.md`,
`artifacts/verification_agent_b.md`, compact metrics, command logs, and
artifact paths. Write access is read-only unless a subagent is explicitly
assigned a bounded implementation fix in the target module, focused runner
tests, integration tests named in the intended write set, or package-local
artifacts.

Subagent requirement: this package requires spawning `comparator_suite_runner`
for heavy batch/closure/comparator runs, including full-workspace CRAP/coverage
after implementation, `cargo nextest run --workspace --profile full`,
comparator suites, and population/fixture batches. Do not run those heavy gates
locally on the parent model unless the subagent is unavailable; if unavailable,
record command-level evidence before running locally.

## Scope

In scope:

- characterization tests required to make Lane D shadow helpers safe to
  refactor;
- behavior-preserving helper extraction or control-flow simplification inside
  the target module;
- local private helpers needed by the extraction;
- focused runner unit, source-guard, or integration tests proving branch,
  validation, numeric, fail-closed, and output-identity behavior;
- package artifacts and prompt material.

Out of scope:

- Lane D activation/default policy, environment-selector, or mutual-exclusion
  semantic changes;
- formula, coefficient, threshold, unit, shape, profile-slot, or finite-guard
  changes;
- public output, manifest schema, serialization, CLI, or fixture semantic
  changes;
- contract amendments or new process authority;
- fallback wrappers that mask invalid Lane D or direct-publication state;
- broad H2637 fixture rewrites or unrelated runner cleanup;
- unrelated active package work.

## Intended Write Set

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs` only if unit-test
  characterization requires crate-local runner surfaces
- `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`
  only if source-guard coverage needs a narrow binding update
- `tests/integration/laned_shadow_h2637.rs` only if output identity or
  fail-closed behavior cannot be proven by unit tests
- package-local artifacts under
  `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/**`
- `docs/work-packages/README.md` after closure if catalog update is needed

Do not edit unrelated dirty files. If the target path is already dirty from
unrelated work, stop before implementation and record a global/process hold.

Line-count note: the target starts at `706` lines, below the 2000-line WARN
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
   - tier assignment (`science-sensitive diagnostic/runtime`);
   - line and region threshold status;
   - per-function 75% region-floor status or disposition;
   - applicable obligation-to-test binding for `SC-OFEROUTE-001`, especially
     `INV-OFEROUTE-010`, `INV-OFEROUTE-012`, and the Lane D shadow/active
     selector rows.
4. Record the behavior oracle and command evidence in
   `artifacts/characterization.md`.

### Phase C - Refactor

1. Cover the high-CRAP helpers before decomposition:
   `LanedShadowCollector::observe_row`,
   `LanedShadowCollector::validate_lane_day_operands`, and
   `LanedShadowCollector::commit_day`.
2. Evaluate `LanedShadowCollector::finalize` because it starts at the exact
   CRAP threshold (`30.0`) and can regress if helper extraction changes
   coverage/complexity.
3. Extract cohesive branch predicates, result constructors, guard helpers, or
   commit-day bookkeeping clusters one at a time.
4. Preserve formulas, branch ordering, short-circuit behavior, error messages
   where asserted, floating-point accumulation/order where material, profile
   accounting, and finite guards.
5. Run focused tests after each meaningful extraction.
6. Stop editing when every owned production function is `<= 30` or explicitly
   dispositioned.
7. Record the implementation in `artifacts/implementation.md`.

### Phase D - Metrics And Gates

1. Re-run coverage and CRAP with the same method as Phase A or a documented
   targeted equivalent when full-workspace coverage is blocked by unrelated
   coverage-instrumented tests.
2. Record `artifacts/crap-after.md` and `artifacts/coverage-after.md`.
3. Record numeric/API/output identity in `artifacts/numeric-equivalence.md`.
4. Record `.rs` line-count governance in
   `artifacts/line-count-governance.md`.
5. Run and record:
   - `git diff --check`
   - markdown/doc lint for touched docs
   - focused openwepp-runner tests
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
numeric/output-identity breaks, target-file crossing the 3000-line blocker
without a valid refactor, and inability to create required scaffold/completion
commits. Rollback only current-package implementation edits and preserve
package evidence; do not revert unrelated user changes or active-package work.

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
- behavior identity is proven by focused tests and appropriate numeric/API or
  output evidence;
- protected outputs and manifest semantics are unchanged unless the package
  holds for a non-CQR follow-on;
- no current-scope gate is deferred without hold classification;
- dual review findings are dispositioned and accepted findings are fixed;
- dual verification passes;
- completion or hold commit exists before the next target package starts.

Final status options:

- `EXECUTED-COMPLETE-CQR-NIGHTLY`
- `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-*`
- `EXECUTED-HOLD-CQR-NIGHTLY-GLOBAL-*`
