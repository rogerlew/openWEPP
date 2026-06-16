# CQR36 - Watershed Impoundment Parser Complexity Closure

Status: complete-with-warnings

Package type: code-quality refactor

## Objective

Close the current CQR36 target in
`crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`,
originally tracked as rank 30 with snapshot CRAP `220`, CC `73`, and coverage
`70%`, so the live target function and any newly extracted helpers have CRAP
`<= 30`.

## Rationale

The watershed impoundment parser owns typed parsing of `.imp` inputs and feeds
watershed runtime state for impoundment/channel execution. The CQR36 package
must reduce complexity without changing public parser API, stable contract error
IDs, parser compatibility, branch arity behavior, typed domain guards, parsed
output shapes, or downstream runtime semantics.

## Quality Dimension

- Dimension: cyclomatic-complexity / CRAP burn-down.
- Closure metric: current target function and any newly extracted helpers have
  CRAP `<= 30` using `cargo crap` against package LCOV.
- Supporting metrics: before/after LCOV, before/after CRAP, target identity,
  line counts, suppression census, parser behavior equivalence, and full gates.

## Included Scope

- Fresh before/after LCOV and CRAP capture for the target file.
- Focused characterization before production refactor if current tests do not
  freeze target branches.
- Behavior-preserving private helper extraction for the scoped target function.
- Package catalog update in `docs/work-packages/README.md`.
- Package artifacts, dual reviews, dual verification, disposition, worker
  handoff, package commit/push, and tracker update after push.

## Excluded Scope / Protected Boundaries

- No public API changes.
- No stable contract error ID changes.
- No parser error variant, field, message, branch arity, compatibility warning,
  strict/compatibility mode, domain guard, invariant, ordering, or source
  behavior changes.
- No parsed output field, collection order, branch comment, fixture, runtime
  projection, downstream kernel, unit, alias, or symbol behavior changes.
- No dependency changes.
- No unrelated parser cleanup or formatter-only churn.

## Parser and Runtime Acceptance Gate

This package is parser/runtime-boundary affecting because `.imp` parser output
is consumed by watershed runtime input projection and watershed impoundment
kernel tests. The acceptance gate is strict behavior preservation:

- preserve all public and crate-visible signatures;
- preserve stable `IMP-E-*` and `IMP-W-*` IDs;
- preserve parser compatibility and strict-mode behavior;
- preserve branch payload ordering and vector consumption semantics;
- preserve all parsed numeric values and output shapes;
- stop and hold if closure requires changing parser authority or runtime
  semantics.

## Intended Write Set

- `docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`
- `tests/integration/infile_watershed_impoundment_parser_contract.rs` only if
  characterization coverage is needed before production refactor.

## Dependencies

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-burndown-execplan.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/codex_exec_plans.md`

## Phase Plan

### Phase A - Intake, Baseline, and Surface Freeze

- Capture target-file line count and suppression census.
- Generate before LCOV and before `cargo-crap` JSON.
- Identify the live target function from current metrics.
- Record protected API, error ID, branch parsing, compatibility, and runtime
  projection surfaces.

### Phase B - Precondition and Characterization

- Run existing focused parser/runtime tests before production edits.
- Add characterization before refactor if current tests do not freeze selected
  parser branches or display/error behavior.

### Phase C - Behavior-Preserving Decomposition

- Extract cohesive private helpers from the scoped target function only.
- Preserve statement order, branch order, vector consumption order, typed errors,
  parsed values, output construction, and public API.

### Phase D - Validation and Evidence

- Re-run focused tests after each production edit.
- Re-run LCOV and `cargo-crap`; target and extracted helpers must be `<= 30`.
- Run the required closure gates:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
  5. `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001 --format json`
  6. `git diff --check`

### Phase E - Review, Verification, Disposition, Commit, and Push

- Complete dual local review artifacts.
- Complete dual local verification artifacts.
- Complete disposition and worker handoff.
- Commit and push the package write set, then update the CQR ExecPlan tracker.

## Exit Criteria

- Current CQR36 target function and any newly extracted helpers have CRAP
  `<= 30`.
- Target-file coverage is not regressed relative to the package baseline.
- No public API, stable error ID, parser compatibility, branch arity, typed
  guard, output shape, runtime projection, or downstream behavior change is
  introduced.
- Required closure gates are run and recorded.
- No touched non-exempt `.rs` file is at or above `3000` lines.
- No review finding remains undispositioned.
- Package commit and tracker update are pushed before the ExecPlan row is
  checked off.

## Review and Verification

This package requires dual independent local review and dual independent local
verification before disposition.
