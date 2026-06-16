# CQR29 - Guard Errors Complexity Closure

Status: complete-with-warnings

Package type: code-quality refactor

## Objective

Close the current CQR29 target in
`crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`,
originally tracked as rank 23 with snapshot CRAP `272`, CC `16`, and coverage
`0%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The target file owns kernel-facing typed guard errors, stable error IDs,
boundary classes, and display strings. This is kernel-affecting support code:
changes must preserve public enum shape, error IDs, display text, boundary
classification, and typed failure behavior.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and any newly extracted helpers have
  CRAP `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, before/after CRAP, target identity,
  line counts, suppression census, behavior equivalence, reviews,
  verification, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Characterization coverage before production refactor when needed.
- Behavior-preserving private helper extraction for the scoped target if live
  metrics require it.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public enum variant changes.
- No error ID, display text, boundary class, typed error, alias, symbol, parser
  compatibility, output behavior, or science-contract behavior changes.
- No dependency changes.
- No unrelated hydrology cleanup.

## Intended Write Set

- `docs/work-packages/20260615-cqr29-guard-errors-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
  only if live metrics require behavior-preserving refactor.
- Focused tests only if characterization coverage is required before refactor.

## Dependencies

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the live target function from current metrics.
- Record protected error IDs, display text, enum shape, and boundary classes.

### Phase B - Precondition and Focused Characterization

- Run existing focused tests before production edits when production edits are
  needed.
- Add characterization before refactor only if current tests do not freeze the
  selected branches.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers only if live metrics require edits.
- Preserve public enum variants, display text, error IDs, boundary classes, and
  typed error behavior.

### Phase D - Validation and Evidence

- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr29-guard-errors-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR29 target function and any newly extracted helpers have CRAP
  `<= 30`: satisfied by `Wb11HydrologyKernelGuardError::fmt` CRAP `1.0` and
  extracted helpers at or below `8.000751314800901`.
- Target-file coverage is not regressed relative to the package baseline.
- Public enum variants, display text, error IDs, boundary classes, and typed
  error behavior are unchanged.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Disposition Summary

Ran: CQR29 added focused characterization for all
`Wb11HydrologyKernelGuardError` variants before production refactor, then
split the prior `Display::fmt` matcher into private display-part helpers.
Public enum shape, error IDs, boundary classes, and display strings are
unchanged.

Warn: `cargo crap` emitted the same LCOV source-map warning class observed in
prior CQR rows: 126 source files had no matching LCOV entry.
