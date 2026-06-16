# CQR25 - Runner Intake Lane Setup Complexity Refactor

Status: complete-with-warnings

Package type: code-quality refactor (behavior-preserving)

## Objective

Decompose the current CQR25 target in
`crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`,
originally tracked as rank 19 with snapshot CRAP `305`, CC `113`, and coverage
`75%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The runner intake and lane setup path owns runfile ingestion, sidecar policy,
runtime-surface assembly, scheduler execution, and output publication
handoffs. CQR25 must reduce local complexity without changing public API,
manifest schema, sidecar behavior, parser compatibility, runtime symbols,
units, output formulas, typed guards, or science-contract behavior.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and newly extracted helpers have CRAP
  `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, target identity, line counts,
  suppression census, public API parity, behavior equivalence, and full gates.

## Included Scope

- Fresh before/after LCOV and before/after `cargo-crap` capture for the target
  file.
- Focused characterization before production refactor when needed.
- Private behavior-preserving helper extraction in
  `00_runner_intake_and_lane_setup.rs`.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API, manifest schema, sidecar policy, dependency, parser
  compatibility, publication schema, runtime symbol, unit, formula, float
  expression order, typed guard, or science-contract behavior changes.
- No unrelated runner, parser, output, scheduler, or kernel cleanup.

## Deliverables

1. Source refactor:
   - `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
2. Focused tests if characterization is required.
3. Package catalog update:
   - `docs/work-packages/README.md`
4. Package artifacts under `artifacts/`.

## Intended Write Set

- `docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- Focused tests under existing runner test paths if characterization is
  required before production refactor.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`
- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the current target function from live metrics.
- Record protected manifest, sidecar, parser, publication, and runtime symbol
  surfaces.

### Phase B - Precondition and Focused Characterization

- Run existing focused tests before production edits.
- Add targeted characterization before production decomposition if current tests
  do not freeze selected branches.
- Run focused tests after characterization and before production refactor.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers as needed.
- Preserve formula operand order, public signatures, typed status behavior,
  publication shape, runtime symbols, manifest schema, and science-contract
  behavior.
- Do one quality dimension only: CRAP/cyclomatic decomposition.

### Phase D - Validation and Evidence

- Run focused tests after the refactor.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts with finding disposition.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR25 target function and any newly extracted helpers have CRAP
  `<= 30`. Final target CRAP: `12.4198250729`.
- Target-file coverage is not regressed relative to the package baseline.
- Focused characterization passes before and after production refactor when
  characterization is added.
- No public API, manifest schema, sidecar policy, runtime symbol, alias, unit,
  stable status/error, parser compatibility, publication schema, or
  science-contract behavior change is introduced.
- Required closure gates are run and recorded with exit codes.
- Public API surface parity is recorded with no intentional deltas.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Completion Evidence

Ran: before LCOV and CRAP captured target `execute_hillslope_run` at line 764
with CC `113`, coverage `75.29722589167768%`, and CRAP
`305.483748671`.

Ran: after LCOV and CRAP captured target `execute_hillslope_run` at line 2292
with CC `12`, coverage `85.71428571428571%`, and CRAP
`12.4198250729`. No target-file CRAP row is above `30`.

Ran: required cargo gates passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

Ran: required documentation and diff gates passed:

- `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr25-runner-intake-lane-setup-complexity-001 --format json`
- `git diff --check`

Static: warnings are limited to known `cargo crap` LCOV source-map warnings for
test/support source files not represented in the LCOV report. The target file
has complete before/after entries.

## Review and Verification

This package requires dual independent local review and dual independent local
verification before disposition.
